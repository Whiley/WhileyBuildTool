package basic;

import java.io.IOException;
import java.util.List;

import jbuildgraph.util.Trie;
import jbuildstore.core.Key;
import jcmdarg.core.Command;
import wy.lang.Environment;

public class Interpreter implements Command<Boolean> {
	private Environment environment;

	public Interpreter(Environment env) {
		this.environment = env;
	}

	@Override
	public Boolean execute() {
		try {
			// Find and execute all binary basic files
			List<Key<Trie,BasicFile>> files = environment.getRepository().match(k -> k.contentType().equals(BasicFile.ContentType));
			//
			for(Key<Trie,BasicFile> f : files) {
				System.out.println("GOT: " + f.id());
				BasicFile bf = environment.getRepository().get(f);
				// Do something
			}
			//
			return true;
		} catch(IOException e) {
			e.printStackTrace();
			return false;
		}
	}
}
