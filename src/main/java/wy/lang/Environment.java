package wy.lang;

import jbuildgraph.core.Build.Artifact;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content;
import jbuildstore.util.HashMapStore;

public class Environment {
	/**
	 * Plugin environment provides access to information sourced from the plugins,
	 * such as available content-types, commands, etc.
	 */
	private final Plugin.Environment env;
	/**
	 * The main repository for storing build artifacts and source files which is
	 * properly versioned.
	 */
	private final Content.Store<Trie, Artifact> repository;
	/**
	 * The working directory where build artifacts are projected, etc.
	 */
	private final Content.Store<Trie, Content> workingRoot;

	public Environment(Plugin.Environment env, Iterable<Artifact> entries, Content.Store<Trie, Content> workingRoot) {
		this.env = env;
		this.repository = new HashMapStore<>();
		this.workingRoot = workingRoot;
		// Initialise store
		for (Artifact b : entries) {
			repository.put(b.getPath(), b);
		}
	}

}
