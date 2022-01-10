package basic;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;

import basic.BasicFile.Expr;
import basic.BasicFile.Stmt;
import jbuildgraph.core.Build;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content;
import jbuildstore.core.Key;
import jsynheap.util.AbstractCompilationUnit.Tuple;

public class CompileTask implements Build.Task {
	@Override
	public boolean apply(Content.Store<Trie> store) {
		try {
			Trie filename = Trie.fromString("input");
			// Determine source & target keys
			Key<Trie, SourceFile> src_k = new Key.Pair<>(filename, SourceFile.ContentType);
			Key<Trie, BasicFile> bin_k = new Key.Pair<>(filename, BasicFile.ContentType);
			SourceFile source = store.get(src_k);
			BasicFile bf = new BasicFile();
			ArrayList<Stmt> stmts = new ArrayList<Stmt>();
			//
			for (String line : getLines(source)) {
				String[] splits = line.split(" ");
				//
				if (splits.length > 0) {
					int lineno = Integer.parseInt(splits[0]);
					Stmt s = parseStatement(lineno,splits);
					stmts.add(bf.allocate(s));
				}
			}
			// set the root!
			bf.setRootItem(new Tuple<>(stmts));
			//
			store.put(bin_k, bf);
			//
			return true;
		} catch (IOException e) {
			// FIXME: this is clearly a problem.
			throw new RuntimeException();
		}
	}

	private Stmt parseStatement(int lineno, String[] splits) {
		// Determine the statement type
		String cmd = splits[1];
		// Dispatch accordingly
		switch(cmd) {
		case "PRINT":
			return parsePrintStatement(lineno,splits);
		case "GOTO":
			return parseGotoStatement(lineno,splits);
		default:
			throw new IllegalArgumentException("Unknown statement encountered");
		}
	}

	private Stmt parseGotoStatement(int lineno, String[] splits) {
		// FIXME: there's plenty that could go wrong here.
		return new Stmt.Goto(lineno,Integer.parseInt(splits[2]));
	}

	private Stmt parsePrintStatement(int lineno, String[] splits) {
		if(splits.length != 3) {
			throw new IllegalArgumentException("invalid print statement");
		} else {
			// Parse the expression being printed
			Expr expr = parseExpression(splits[2]);
			// Done
			return new Stmt.Print(lineno,expr);
		}
	}

	private Expr parseExpression(String str) {
		// FIXME: this is a temporary hack!
		return new Expr.Constant(str);
	}

	public static List<String> getLines(SourceFile source) {
		String[] lines = new String(source.getBytes(StandardCharsets.US_ASCII)).split("\n");
		return Arrays.asList(lines);
	}
}
