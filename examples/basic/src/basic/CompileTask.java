package basic;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import basic.BinaryFile.Stmt;
import jbfs.core.Build;
import jbfs.util.Pair;
import jbfs.util.Trie;

public class CompileTask implements Build.Task {
	private final Trie path;
	private final SourceFile source;

	public CompileTask(Trie path, SourceFile source) {
		this.path = path;
		this.source = source;
	}

	@Override
	public Trie getPath() {
		return path;
	}

	@Override
	public Type<? extends Build.Artifact> getContentType() {
		return BinaryFile.ContentType;
	}

	@Override
	public List<? extends Build.Artifact> getSourceArtifacts() {
		return Arrays.asList(source);
	}

	@Override
	public Pair<Build.SnapShot, Boolean> apply(Build.SnapShot t) {
		// FIXME: should read from snapshot or just use source?
		ArrayList<BinaryFile.Stmt> stmts = new ArrayList<>();
		//
		for(String line : getLines(source)) {
			stmts.add(parseStatement(line));
		}
		//
		Build.SnapShot snap = t.put(new BinaryFile(path, source, stmts));
		//
		return new Pair<>(snap, true);
	}

	private Stmt parseStatement(String line) {
		System.out.println("GOT: " + line);
		return null;
	}

	public static List<String> getLines(SourceFile source) {
		String[] lines = new String(source.getBytes()).split("\n");
		return Arrays.asList(lines);
	}
}
