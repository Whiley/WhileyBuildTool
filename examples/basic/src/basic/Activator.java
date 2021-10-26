package basic;

import wybt.lang.Plugin;
import wybt.lang.Plugin.Context;

public class Activator implements Plugin.Activator {

	@Override
	public Plugin start(Context context) {
		System.out.println("BASIC PLUGIN STARTING!");
		//
		return new Plugin() {
			
		};
	}

	@Override
	public void stop(Plugin module, Context context) {
		System.out.println("BASIC PLUGIN FINISHING!");		
	}

}
