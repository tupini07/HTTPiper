// https://github.com/alecthomas/participle
package main

import (
	"fmt"
	"os"

	"parser"

	"github.com/antlr/antlr4/runtime/Go/antlr"
)

type BaseVisitor struct {
	*parser.BasedumdumVisitor
}

func NewBaseVisitor() *BaseVisitor {
	return new(BaseVisitor)
}

func (this *BaseVisitor) VisitProgram(ctx *parser.ProgramContext) {
	fmt.Println(ctx.GetText())
}

func (this *BaseVisitor) VisitOration(ctx *parser.OrationContext) {
	fmt.Println(ctx.GetText())
}

func (v *BaseVisitor) Visit(tree antlr.ParseTree) interface{} {
	return tree.Accept(v)
}

func (v *BaseVisitor) VisitChildren(node antlr.RuleNode) interface{} {
	for _, child := range node.GetChildren() {
		child.(antlr.ParseTree).Accept(v)
	}
	return nil
}

func main() {
	input, _ := antlr.NewFileStream(os.Args[1])
	lexer := parser.NewdumdumLexer(input)
	stream := antlr.NewCommonTokenStream(lexer, 0)
	p := parser.NewdumdumParser(stream)
	p.BuildParseTrees = true

	NewBaseVisitor().Visit(p.Program())
}
