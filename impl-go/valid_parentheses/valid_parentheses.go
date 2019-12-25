package valid_parentheses

import "container/list"

func isValid(s string) bool {
	stack := list.New()
	for _, c := range s {
		switch c {
		case '(':
			stack.PushBack(')')
		case '{':
			stack.PushBack('}')
		case '[':
			stack.PushBack(']')
		default:
			if stack.Len() == 0 {
				return false
			}
			e := stack.Back()
			stack.Remove(e)
			if v := e.Value.(int32); v != c {
				return false
			}
		}
	}
	return stack.Len() == 0
}
