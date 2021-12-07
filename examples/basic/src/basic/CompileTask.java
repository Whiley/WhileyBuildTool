package basic;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import basic.BinaryFile.Stmt;
import jbuildgraph.core.Build;
import jbuildgraph.core.Build.Artifact;
import jbuildgraph.util.Pair;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content;
import jbuildstore.core.Content.Ledger;

public class CompileTask implements Build.Task {
	private static final Pattern LINE_MATCH = Pattern.compile("[a-zA-Z0-9/\\\\_.:]+");
	private final Trie path;
	private final SourceFile source;

	public CompileTask(Trie path, SourceFile source) {
		this.path = path;
		this.source = source;
	}

	@Override
	public boolean apply(Content.Ledger<Trie, Artifact> ledger) {
		// FIXME: should read from snapshot or just use source?
		ArrayList<BinaryFile.Stmt> stmts = new ArrayList<>();
		//
		for(String line : getLines(source)) {
			stmts.add(parseStatement(line));
		}
		//
//		ledger.put(new BinaryFile(path, source, stmts));
//		//
//		return true;
		//
		throw new UnsupportedOperationException();
	}

	@Override
	public List<Artifact> requires() {
		throw new UnsupportedOperationException();
	}

	@Override
	public List<Artifact> ensures() {
		throw new UnsupportedOperationException();
	}

	private Stmt parseStatement(String line) {
		Matcher matcher = LINE_MATCH.matcher(line);
		//
		if (matcher.matches()) {
			int lineno = Integer.parseInt(matcher.group(0));
			String rest = matcher.group(1);
			System.out.println("GOT LINE: " + lineno);
			//
			return null;
		} else {
			throw new RuntimeException("error reporting?");
		}
	}

	public static List<String> getLines(SourceFile source) {
		String[] lines = new String(source.getBytes()).split("\n");
		return Arrays.asList(lines);
	}
}
