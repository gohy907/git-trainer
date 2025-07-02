package main

import (
    "fmt"
    "os"

    tea "github.com/charmbracelet/bubbletea"
)

type choice struct {
    title       string
    description []string
}

type model struct {
    choices  			[]choice
    cursor   			int

	confirmMenuOpen 	bool
	confirmMenuCursor 	int
}

func initialModel() model {
    return model{
        choices: []choice{
            {"Сценарий 1", []string{"Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "Donec finibus, tortor nec commodo iaculis, metus."}},
            {"Сценарий 2", []string{"Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "Ut efficitur, purus ut venenatis viverra, leo."}},
            {"Сценарий 3", []string{"Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "Sed cursus efficitur viverra. Proin eget fringilla."}},
            {"Сценарий 4", []string{"Aboba"}},
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
			if m.confirmMenuOpen {
				if m.confirmMenuCursor == 1 {
					m.confirmMenuOpen = false
				}
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

		s += fmt.Sprintf("\n   %s %s\n", yes, no)

	} else {
		s += "Выберите сценарий\n\n"
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
    s += "\nPress q to quit.\n"
    return s
}

func main() {
    p := tea.NewProgram(initialModel())
    if _, err := p.Run(); err != nil {
        fmt.Printf("Alas, there's been an error: %v", err)
        os.Exit(1)
    }
}
