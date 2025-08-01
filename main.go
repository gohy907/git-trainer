package main

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"os"
	"os/exec"
	"strconv"
)

type choice struct {
	title       string
	description []string
}

type model struct {
	choices []choice
	cursor  int

	confirmMenuOpen   bool
	confirmMenuCursor int

	executing bool
}

func initialModel() model {
	return model{
		choices: []choice{
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
			if !m.confirmMenuOpen && m.cursor > 0 {
				m.cursor--
			}
		case "down", "j":
			if !m.confirmMenuOpen && m.cursor < len(m.choices)-1 {
				m.cursor++
			}
		case "enter":
			if m.confirmMenuOpen {

				if m.confirmMenuCursor == 0 && !m.executing {
					containerToRun = m.cursor + 1
					return m, tea.Quit
				}

				m.confirmMenuOpen = false

			} else {
				m.confirmMenuOpen = true
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
	if m.confirmMenuOpen {
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
		s += "\n"
	}

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
