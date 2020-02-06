import antlr4

from piper.aparser import dumdumLexer, dumdumParser, dumdumVisitor


class CustomVisitor(dumdumVisitor.dumdumVisitor):
    # Visit a parse tree produced by dumdumParser#program.
    def visitProgram(self, ctx: dumdumParser.dumdumParser.ProgramContext):
        return {
            "full_program": ctx.getText(),
            "orations": [self.visitOration(x) for x in ctx.oration()],
        }

    # Visit a parse tree produced by dumdumParser#oration.
    def visitOration(self, ctx: dumdumParser.dumdumParser.OrationContext):
        return {
            "full_oration": ctx.getText(),
            "expr": self.visitExpr(ctx.expr()),
        }

    # Visit a parse tree produced by dumdumParser#expr.
    def visitExpr(self, ctx: dumdumParser.dumdumParser.ExprContext):
        return {
            "full_expression": ctx.getText(),
            "method": self.visitMethod(ctx.method()),
            "alpha": ctx.getTokens(dumdumParser.dumdumParser.ALPHA)[0].getText()
        }

    # Visit a parse tree produced by dumdumParser#method.
    def visitMethod(self, ctx: dumdumParser.dumdumParser.MethodContext):
        return ctx.getText()


if __name__ == "__main__":

    lexer = dumdumLexer.dumdumLexer(antlr4.FileStream("test.dum"))

    stream = antlr4.CommonTokenStream(lexer)

    parser = dumdumParser.dumdumParser(stream)

    tree = parser.program()
    a = CustomVisitor().visit(tree)
