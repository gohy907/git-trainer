package main

import (
	"fmt"
)

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
	for i := range len(m.choices) {
		choice := m.choices[choiceType(i)]
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
	choice := m.choices[choiceType(m.cursor)]
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
