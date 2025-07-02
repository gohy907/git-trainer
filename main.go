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
    choices  []choice
	choosed  int
    cursor   int
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
            if m.cursor > 0 {
                m.cursor--
            }
        case "down", "j":
            if m.cursor < len(m.choices)-1 {
                m.cursor++
            }
		case "enter":
			if m.choosed == m.cursor{
				m.choosed = -1 
			} else {
				m.choosed = m.cursor
			}
		}
    }
    return m, nil
}

func (m model) View() string {
    s := "What should we buy at the market?\n\n"
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
			if (m.cursor == m.choosed){
				s += fmt.Sprintf("aboba?")
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
