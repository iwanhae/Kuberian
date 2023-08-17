package main

import "sort"

type Functions []Function

var _ sort.Interface = Functions{}

// Len implements sort.Interface.
func (f Functions) Len() int {
	return len(f)
}

// Less implements sort.Interface.
func (f Functions) Less(i int, j int) bool {
	if f[i].File < f[j].File {
		return true
	} else if f[i].File > f[j].File {
		return false
	}
	return f[i].Line.From < f[j].Line.From
}

// Swap implements sort.Interface.
func (f Functions) Swap(i int, j int) {
	f[i], f[j] = f[j], f[i]
}

type Function struct {
	ID   uint64      `json:"id"`
	Name string      `json:"name"`
	File string      `json:"file"`
	Code string      `json:"code"`
	Line FunctionPos `json:"line"`
}

type FunctionPos struct {
	From int `json:"from"`
	To   int `json:"to"`
}
