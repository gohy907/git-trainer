package main

import (
	"encoding/json"
	"fmt"
	"os"
)

func defaultConfig(tasks []task) {
	taskMap := map[int]task{}

	for i, task := range tasks {
		taskMap[i] = task
	}

	json, err := json.Marshal(taskMap)

	if err != nil {
		fmt.Println("Ошибка: ", err)
	}

	err = os.WriteFile("config.json", json, 0644)
	if err != nil {
		fmt.Println("Ошибка: ", err)
	}
}

func checkForConfig() {
	_, err := os.ReadFile("config.json")
	if err != nil {
		if os.IsNotExist(err) {
			defaultConfig(initTasks())
		}
	}
}

func getTasksFromConfig() []task {
	jsonBytes, err := os.ReadFile("config.json")
	if err != nil {
		fmt.Println("Ошибка: ", err)
	}
	var tasks []task
	err = json.Unmarshal(jsonBytes, &tasks)
	if err != nil {
		fmt.Println("Ошибка: ", err)
	}

	return tasks
}
