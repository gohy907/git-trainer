package main

import (
	tea "github.com/charmbracelet/bubbletea"
)

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
			if m.choices[choiceType(m.cursor)].needConfifmation {
				if m.confirmMenuOpen {
					if m.confirmMenuCursor == 0 {
						if choiceType(m.cursor) == restartTask {
							//restartTask()
						}
					}

					m.confirmMenuOpen = false

				} else {
					m.confirmMenuOpen = true
				}
			} else {
				// TODO: сделать проверку задания. Пока это единственный выбор без подтверждения
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
