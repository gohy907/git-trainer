package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"

	tea "github.com/charmbracelet/bubbletea"
)

type Information struct {
	Title       string
	Description []string `json:"description"`
}

type Task struct {
	TaskNumber  int      `json:"taskNumber"`
	Description []string `json:"description"`
}

func parseJsonAsTask(name string) Task {
	file, _ := os.Open(name)
	defer file.Close()

	bytes, _ := io.ReadAll(file)

	var task Task
	_ = json.Unmarshal(bytes, &task)

	return task
}

var binPath, _ = os.Executable()
var descJsonPath = filepath.Join(filepath.Dir(binPath), "description.json")

var task = parseJsonAsTask(descJsonPath)

type model struct {
	choices map[choiceType]choice
	cursor  int

	confirmMenuOpen   bool
	confirmMenuCursor int

	informations []Information

	taskNumber int
}

func initialModel() model {
	return model{
		informations: []Information{
			{"Описание задания:", task.Description},
		},
		choices: map[choiceType]choice{
			checkForCompletion: {
				title:            "Проверить задание:",
				description:      []string{"Проверить, выполнены ли цели задания"},
				needConfifmation: false,
			},

			restartTask: {
				title:            "Перезагрузить задание:",
				description:      []string{"Начать задание с нуля, поможет, если Вы застряли"},
				needConfifmation: true,
			},
		},
	}
}

func (m model) Init() tea.Cmd {
	return nil
}

func main() {

	p := tea.NewProgram(initialModel())
	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}
