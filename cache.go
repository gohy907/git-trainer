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
