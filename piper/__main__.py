import antlr4

from piper.aparser import dumdumLexer, dumdumParser, dumdumVisitor

from piper import aparser


class CustomVisitor(dumdumVisitor.dumdumVisitor):
    # Visit a parse tree produced by dumdumParser#program.
    def visitProgram(self, ctx: dumdumParser.):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by dumdumParser#oration.
    def visitOration(self, ctx:dumdumParser.OrationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by dumdumParser#expr.
    def visitExpr(self, ctx:dumdumParser.ExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by dumdumParser#method.
    def visitMethod(self, ctx:dumdumParser.MethodContext):
        return self.visitChildren(ctx)



if __name__ == "__main__":
    
    lexer = dumdumLexer.dumdumLexer(antlr4.FileStream("test.dum"))

    stream = antlr4.CommonTokenStream(lexer)

    parser = dumdumParser.dumdumParser(stream)

    tree = parser.program()
    import pudb; pudb.set_trace()