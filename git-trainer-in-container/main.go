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

func parseJsonAsInformation(name string) Information {
	file, _ := os.Open(name)
	defer file.Close()

	bytes, _ := io.ReadAll(file)

	var info Information
	_ = json.Unmarshal(bytes, &info)

	return info
}

var binPath, _ = os.Executable()
var descFilePath = filepath.Join(filepath.Dir(binPath), "description.json")
var InfoAboutTask Information = parseJsonAsInformation(descFilePath)

type model struct {
	choices map[choiceType]choice
	cursor  int

	confirmMenuOpen   bool
	confirmMenuCursor int

	informations []Information
}

func initialModel() model {
	return model{
		informations: []Information{
			{"Описание задания:", InfoAboutTask.Description},
		},
		choices: map[choiceType]choice{
			checkForCompletion: {
				title:            "Описание задания:",
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
	fmt.Printf(descFilePath)

	p := tea.NewProgram(initialModel())
	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}
