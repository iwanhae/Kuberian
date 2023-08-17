package main

import (
	"encoding/json"
	"go/ast"
	"go/parser"
	"go/token"
	"io/fs"
	"log"
	"os"
	"path"
	"path/filepath"
	"sort"
	"strings"
)

func main() {
	results := Functions{}

	// Parse
	filepath.Walk("./kubernetes", func(file string, info fs.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if info.IsDir() {
			return nil
		}
		if path.Ext(file) != ".go" {
			return nil
		}
		for _, s := range []string{
			"fake",         // test code
			"test",         // test code
			"zz_generated", // generated code
			"generated",    // generated code
			"api.pb.go",    // generated code
			"vendor",       // external libs
			"third_party",  // external libs
			"sample",       // example code
			"example",
		} {
			if strings.Contains(file, s) {
				return nil
			}
		}
		results = append(results, parseSrc(file)...)
		return nil
	})

	// Sort
	sort.Sort(results)

	// Print
	repoId := 1 /* id of the kubernetes/kubernetes@v1.27.4 */
	id := repoId * 100_000_000
	for _, f := range results {
		f.ID = uint64(id)
		id += 1
		json.NewEncoder(os.Stdout).Encode(f)
	}
}

func parseSrc(filename string) []Function {
	s, err := os.ReadFile(filename)
	if err != nil {
		panic(err) // there will be no err
	}
	lines := strings.Split(string(s), "\n")

	f := []Function{}
	fset := token.NewFileSet()
	node, err := parser.ParseFile(fset, filename, nil, parser.ParseComments)
	if err != nil {
		log.Fatalf("Error parsing file: %v", err)
	}

	for _, decl := range node.Decls {
		switch d := decl.(type) {
		case *ast.FuncDecl:
			start := fset.Position(d.Pos()).Line
			if d.Doc != nil {
				start = fset.Position(d.Doc.Pos()).Line
			}
			end := fset.Position(d.End()).Line
			name_pos := fset.Position(d.Name.Pos()).Line
			name, _ := strings.CutSuffix(lines[name_pos-1], " {")

			f = append(f, Function{
				Name: name,
				Line: FunctionPos{
					From: start,
					To:   end,
				},
				File: filename[11:],
				Code: strings.Join(lines[start-1:end], "\n"),
			})
		}
	}

	return f
}
