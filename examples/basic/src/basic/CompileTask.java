package basic;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;

import basic.BasicFile.Expr;
import basic.BasicFile.Stmt;
import jbuildgraph.core.Build;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content;
import jbuildstore.core.Key;

public class CompileTask implements Build.Task {
	@Override
	public boolean apply(Content.Store<Trie> store) {
		try {
			Trie filename = Trie.fromString("input");
			// Determine source & target keys
			Key<Trie, SourceFile> src_k = new Key.Pair<>(filename, SourceFile.ContentType);
			Key<Trie, BasicFile> bin_k = new Key.Pair<>(filename, BasicFile.ContentType);
			SourceFile source = store.get(src_k);
			HashMap<Integer,BasicFile.Stmt> stmts = new HashMap<>();
			//
			for (String line : getLines(source)) {
				String[] splits = line.split(" ");
				//
				if (splits.length > 0) {
					int lineno = Integer.parseInt(splits[0]);
					// Sanity check we don't have
					if (stmts.containsKey(lineno)) {
						throw new IllegalArgumentException("duplicate line number encountered!");
					} else {
						stmts.put(lineno, parseStatement(splits));
					}
				}
			}
			//
			store.put(bin_k, new BasicFile(stmts));
			//
			return true;
		} catch (IOException e) {
			// FIXME: this is clearly a problem.
			throw new RuntimeException();
		}
	}

	private Stmt parseStatement(String[] splits) {
		// Determine the statement type
		String cmd = splits[1];
		// Dispatch accordingly
		switch(cmd) {
		case "PRINT":
			return parsePrintStatement(splits);
		case "GOTO":
			return parseGotoStatement(splits);
		default:
			throw new IllegalArgumentException("Unknown statement encountered");
		}
	}

	private Stmt parseGotoStatement(String[] splits) {
		// FIXME: there's plenty that could go wrong here.
		return new Stmt.Goto(Integer.parseInt(splits[2]));
	}

	private Stmt parsePrintStatement(String[] splits) {
		if(splits.length != 3) {
			throw new IllegalArgumentException("invalid print statement");
		} else {
			// Parse the expression being printed
			Expr expr = parseExpression(splits[2]);
			// Done
			return new Stmt.Print(expr);
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
