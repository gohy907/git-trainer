package main

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"os"
	"os/exec"
	"strconv"
)

type task struct {
	title       string
	description []string
}

type model struct {
	choices []task
	cursor  int

	selectActionCursor   int
	selectActionMenuOpen bool

	confirmMenuOpen   bool
	confirmMenuCursor int

	executing bool
}

type actionDescription string

const (
	enter        actionDescription = "Начать задание"
	save         actionDescription = "Отправить на проверку"
	restart      actionDescription = "Перезагрузить задание"
	continueTask actionDescription = "Продолжить задание"
)

type action struct {
	info             actionDescription
	needConfirmation bool
}

var defaultActions = []action{
	{enter, true},
	{save, true},
	{restart, true},
}

func initialModel() model {
	return model{
		choices: []task{
			{"Привет, мир!", []string{"В этой задаче Вам предстоит создать новый Git репозиторий", "и сделать в нём первый коммит"}},
			{"Своих не сдаём", []string{"Последний коммит в этой задаче посеял в коде критический баг", "Вам нужно исправить этот баг, не создавая нового коммита"}},
		},
	}
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case restartMsg:
		// Заново инициализируем интерфейс
		return initialModel(), tea.ClearScreen
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit
		case "up", "k":
			if !m.selectActionMenuOpen && m.cursor > 0 {
				m.cursor--
			} else if !m.confirmMenuOpen && m.selectActionCursor > 0 {
				m.selectActionCursor--
			}
		case "down", "j":
			if !m.selectActionMenuOpen && m.cursor < len(m.choices)-1 {
				m.cursor++
			} else if !m.confirmMenuOpen && m.selectActionCursor < len(defaultActions)-1 {
				m.selectActionCursor++
			}
		case "enter":
			if m.selectActionMenuOpen {
				if m.confirmMenuOpen {

					if m.confirmMenuCursor == 0 && !m.executing {
						containerToRun = m.cursor + 1
						return m, tea.Quit
					}

					m.confirmMenuOpen = false

				} else {
					m.confirmMenuOpen = true
				}
			} else {
				m.selectActionMenuOpen = true
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

func (m model) View() string {

	var s string
	choice := m.choices[m.cursor]

	if m.selectActionMenuOpen && m.confirmMenuOpen {
		actionChoosed := defaultActions[m.selectActionCursor]
		s += "Подтвердите выбор\n\n"
		s += fmt.Sprintf("%s %s\n", ">", choice.title)

		for _, desc := range choice.description {
			s += fmt.Sprintf("    %s\n", desc)
		}
		s += "\n"
		s += fmt.Sprintf("   >> %s\n", actionChoosed.info)

		yes := "     да "
		no := " нет "

		if m.confirmMenuCursor == 0 {
			yes = "    [да]"
		} else {
			no = "[нет]"
		}

		s += fmt.Sprintf("\n   %s %s\n", yes, no)
	} else if m.selectActionMenuOpen {
		s += "Выберите действие\n\n"
		s += fmt.Sprintf("%s %s\n", ">", choice.title)
		for _, desc := range choice.description {
			s += fmt.Sprintf("    %s\n", desc)
		}

		s += "\n"

		for i, action := range defaultActions {
			cursor := "     "
			if m.selectActionCursor == i {
				cursor = "   >>"
			}
			s += fmt.Sprintf("%s %s\n", cursor, action.info)
		}

	} else {
		s += "Выберите задачу\n\n"
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
	}

	s += "\n"

	if m.confirmMenuOpen {
		s += "Нажмите ← или → для навигации, Enter для выбора, q для выхода"
	} else {
		s += "Нажмите ← ↓ ↑ → для навигации, Enter для выбора, q для выхода"
	}
	return s
}

type Cmd struct {
	name string
	args []string
}

type restartMsg struct{}

var p *tea.Program
var containerToRun int

func main() {
	for {
		p := tea.NewProgram(initialModel(), tea.WithAltScreen())
		if _, err := p.Run(); err != nil {
			fmt.Printf("Ошибка: %v\n", err)
			os.Exit(1)
		}

		if containerToRun > 0 {
			runContainer(containerToRun)
			containerToRun = 0
		} else {
			break
		}
	}
}

func runContainer(taskID int) {
	clear := exec.Command("clear")
	err := clear.Run()
	if err != nil {
		fmt.Printf("Ошибка")
	}

	cmd := exec.Command("./run.sh", strconv.Itoa(taskID))
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err = cmd.Run()
	if err != nil {
		fmt.Printf("Ошибка запуска контейнера: %v\n", err)
	}

}

func sendTask(taskID int) {
	task := "task" + strconv.Itoa(taskID)
	taskImage := task + ":$USER_attempt"
	cmd := exec.Command("docker", "commit", task, taskImage,
		"&&", "docker", "save", "-o", fmt.Sprintf("~/.git-trainer/attempts/%s.tar", task+"$USER_attempt"), taskImage,
		"&&", "docker", "rmi", taskImage)

	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err := cmd.Run()
	if err != nil {
		fmt.Printf("Ошибка сохранения контейнера: %v\n", err)
	}

}
