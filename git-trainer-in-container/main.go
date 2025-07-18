package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"

	tea "github.com/charmbracelet/bubbletea"
)

type choice struct {
	title            string
	description      []string
	needConfifmation bool
}

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
	choices []choice
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
		choices: []choice{
			{"Проверить выполнение", []string{"Проверить, выполнены ли цели задания"}, false},
			{"Перезагрузить задание", []string{"Начать задание с нуля, поможет, если Вы застряли"}, true},
		},
	}
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit
		case "up", "k":
			if !m.confirmMenuOpen && m.cursor > 0 {
				m.cursor--
			}
		case "down", "j":
			if !m.confirmMenuOpen && m.cursor < len(m.choices)-1 {
				m.cursor++
			}
		case "enter":
			if m.choices[m.cursor].needConfifmation {
				if m.confirmMenuOpen {

					if m.confirmMenuCursor == 0 {
						enterTask(m.cursor)
					}

					m.confirmMenuOpen = false

				} else {
					m.confirmMenuOpen = true
				}
			} else {
				if m.cursor == 0 {
				}
			}

		case "left", "h":
			if m.confirmMenuCursor > 0 {
				m.confirmMenuCursor--
			}

		case "right", "l":
			if m.confirmMenuCursor < 1 {
				m.confirmMenuCursor++
			}
		}
	}
	return m, nil
}

func viewInformation(m model, s string) string {
	for _, info := range m.informations {
		s += fmt.Sprintf("%s\n", info.Title)
		for _, desc := range info.Description {
			s += fmt.Sprintf("%s\n", desc)
		}
		s += "\n"

	}

	return s
}

func viewChoices(m model, s string) string {
	s += "Выберите действие:\n"
	for i, choice := range m.choices {
		cursor := " "
		if m.cursor == i {
			cursor = ">"
		}
		s += fmt.Sprintf("%s %s\n", cursor, choice.title)
		if m.cursor == i {
			for _, desc := range choice.description {
				s += fmt.Sprintf("    %s\n", desc)
			}
		}
	}
	s += "\n"
	s += "Нажмите ← ↓ ↑ → для навигации, Enter для выбора, q для выхода"

	return s
}

func viewConfirmMenu(m model, s string) string {
	choice := m.choices[m.cursor]
	s += "Подтвердите выбор\n\n"
	s += fmt.Sprintf("%s %s\n", ">", choice.title)
	for _, desc := range choice.description {
		s += fmt.Sprintf("    %s\n", desc)
	}
	yes := " да "
	no := " нет "

	if m.confirmMenuCursor == 0 {
		yes = "[да]"
	} else {
		no = "[нет]"
	}
	s += fmt.Sprintf("\n   %s %s\n\n", yes, no)
	s += "Нажмите ← или → для навигации, Enter для выбора, q для выхода"

	return s
}

func (m model) View() string {
	var s string
	if m.confirmMenuOpen {
		s = viewConfirmMenu(m, s)

	} else {
		s = viewInformation(m, s)
		s = viewChoices(m, s)
	}

	return s
}

type Cmd struct {
	name string
	args []string
}

func newCmd(name string, args ...string) Cmd {
	return Cmd{name: name, args: args}
}

func runCmd(command Cmd) error {
	cmd := exec.Command(command.name, command.args...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

func enterTask(taskNumber int) {
	clear := newCmd("clear")
	run := newCmd("./run.sh", strconv.Itoa(taskNumber))
	runCmd(clear)
	runCmd(run)
	runCmd(clear)
}

func main() {
	fmt.Printf(descFilePath)

	p := tea.NewProgram(initialModel())
	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}
