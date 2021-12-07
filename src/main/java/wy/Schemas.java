// Copyright 2011 The Whiley Project Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
package wy;

import java.util.regex.Pattern;

import jbuildgraph.util.Trie;
import wy.cfg.Configuration;

/**
 * Provides a single point of truth for all schemas used within this tool.
 */
public class Schemas {

    /**
     * Schema for system configuration (i.e. which applies to all users).
     */
    public static Configuration.Schema SYSTEM_CONFIG_SCHEMA = Configuration.fromArray(
            Configuration.UNBOUND_STRING(Trie.fromString("plugins/*"), "list of globally installed plugins", true));

    /**
     * Schema for global configuration (i.e. which applies to all projects for a given user).
     */
    public static Configuration.Schema GLOBAL_CONFIG_SCHEMA = Configuration.fromArray(
            Configuration.UNBOUND_STRING(Trie.fromString("user/name"), "username", false),
            Configuration.UNBOUND_STRING(Trie.fromString("user/email"), "email", false));

    /**
     * Schema for local configuration (i.e. which applies to a given workspace).
     */
    public static Configuration.Schema LOCAL_CONFIG_SCHEMA = Configuration.fromArray(
            Configuration.UNBOUND_STRING_ARRAY(Trie.fromString("workspace/projects"), "list of projects", false));


    /**
     * This determines what files are included in a package be default (i.e. when
     * the build/includes attribute is not specified).
     */
    public static final Object[] DEFAULT_BUILD_INCLUDES = {
            // Include package description by default
            "wy.toml",
            // Include all wyil files by default
            "**/*.wyil",
            // Include all whiley files by default
            "**/*.whiley"
    };

	/**
	 * Schema for packages (i.e. which applies to a single project for a given
	 * user).
	 */
	public static Configuration.Schema PACKAGE = Configuration.fromArray(
			// Required items
			Configuration.UNBOUND_STRING(Trie.fromString("package/name"), "Name of this package", "main"),
			Configuration.UNBOUND_STRING_ARRAY(Trie.fromString("package/authors"), "Author(s) of this package", false),
			Configuration.UNBOUND_STRING(Trie.fromString("package/version"), "Semantic version of this package", false),
			// Build items
			Configuration.UNBOUND_STRING_ARRAY(Trie.fromString("build/platforms"),
					"Target platforms for this package (default just \"whiley\")", new Object[] { "whiley" }),
			Configuration.UNBOUND_STRING_ARRAY(Trie.fromString("build/includes"), "Files to include in package",
					DEFAULT_BUILD_INCLUDES),
			Configuration.UNBOUND_STRING(Trie.fromString("build/main"), "Identify main method", false),
			// Optional items
			Configuration.REGEX_STRING(Trie.fromString("dependencies/*"), "Packages this package depends on", false,
					Pattern.compile("\\d+.\\d+.\\d+")));
}
