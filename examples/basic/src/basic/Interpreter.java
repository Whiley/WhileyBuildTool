package basic;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import basic.BasicFile.Expr;
import basic.BasicFile.Stmt;
import jbuildgraph.util.Trie;
import jbuildstore.core.Key;
import jcmdarg.core.Command;
import jsynheap.util.AbstractCompilationUnit.Tuple;
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
			List<Key<Trie, BasicFile>> files = environment.getRepository()
					.match(k -> k.contentType().equals(BasicFile.ContentType));
			//
			for (Key<Trie, BasicFile> f : files) {
				BasicFile bf = environment.getRepository().get(f);
				Tuple<BasicFile.Stmt> program = (Tuple<BasicFile.Stmt>) bf.getRootItem();
				// FIXME: could generate a cache here
				execute(0, new HashMap<>(), program);
			}
			//
			return true;
		} catch (IOException e) {
			e.printStackTrace();
			return false;
		}
	}

	private void execute(int pc, Map<String, Object> env, Tuple<BasicFile.Stmt> program) {
		while (pc < program.size()) {
			Stmt stmt = program.get(pc);
			switch (stmt.getOpcode()) {
			case BasicFile.STMT_print:
				pc = executeStmtPrint(pc, (Stmt.Print) stmt, env);
				break;
			case BasicFile.STMT_goto:
				pc = executeStmtGoto(pc, (Stmt.Goto) stmt, env, program);
				break;
			default:
				throw new IllegalArgumentException("Unknown statmenet encountered");
			}
		}
	}

	private int executeStmtPrint(int pc, Stmt.Print stmt, Map<String, Object> env) {
		Object o = execute(stmt.getExpr(), env);
		System.out.println(o);
		return pc + 1;
	}

	private int executeStmtGoto(int pc, Stmt.Goto stmt, Map<String, Object> env, Tuple<BasicFile.Stmt> program) {
		int line = stmt.getTarget();
		//
		for(int i=0;i!=program.size();++i) {
			Stmt ith = program.get(i);
			if(ith.getLineNumber() == line) {
				return i;
			}
		}
		//
		throw new IllegalArgumentException("invalid goto target");
	}

	private Object execute(Expr e, Map<String, Object> env) {
		switch(e.getOpcode()) {
		case BasicFile.EXPR_const:
			return executeExprConst((Expr.Constant)e,env);
		default:
			throw new IllegalArgumentException("Unknown expression encountered");
		}
	}

	private Object executeExprConst(Expr.Constant expr, Map<String,Object> env) {
		return expr.getAs(Object.class);
	}
}
