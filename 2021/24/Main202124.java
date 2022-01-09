import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Main202124 {
	static List<Command> commands;
	static Set<Registers> visited;
	static long start;
	static long t;
	static long hits;

	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2021/24/24.txt"));
		start = System.currentTimeMillis();
		commands = new ArrayList<>();
		for (String line : input) {
			String[] tokens = line.split(" ");
			String cmd = tokens[0];
			commandFactory(tokens, cmd);
		}
		visited = new HashSet<>();
		hits = 0;
		long[] registers = new long[4];
		t = 0;
		Long result = foo(0, registers);
		System.out.println(result == null ? "null" : new StringBuilder("" + result).reverse().toString());
		long end = System.currentTimeMillis();
		System.out.format("visited.size: %d, hits: %d\n", visited.size(), hits);
		System.out.format("%d ms\n", end - start);
	}

	private static void commandFactory(String[] tokens, String cmd) {
		switch (cmd) {
		case "inp":
			commands.add(null);
			break;
		case "add":
			commands.add(new CommandAdd(tokens));
			break;
		case "mul":
			commands.add(new CommandMul(tokens));
			break;
		case "div":
			commands.add(new CommandDiv(tokens));
			break;
		case "mod":
			commands.add(new CommandMod(tokens));
			break;
		case "eql":
			commands.add(new CommandEql(tokens));
			break;
		}
	}

	private static Long foo(int idx, long[] registers) {
		if (visited.contains(new Registers(idx, registers[0], registers[1], registers[2], registers[3]))) {
			hits++;
			return null;
		}
		long[] localRegisters = new long[4];
		loop1: for (long d = 1; d < 10; d++) {
			int i = idx;
			setLocalRegisters(registers, localRegisters, d);
			i++;
			while (i < commands.size()) {
				Command command = commands.get(i);
				if (command == null) {
					Long result = foo(i, localRegisters);
					if (result != null) {
						System.out.println(result);
						return result * 10 + d;
					}
					else continue loop1;
				} else {
					command.exec(localRegisters);
				}
				i++;
			}
			if (localRegisters[3] == 0) {
				return d;
			}
		}
		visited.add(new Registers(idx, registers[0], registers[1], registers[2], registers[3]));
		return null;
	}

	private static void setLocalRegisters(long[] registers, long[] localRegisters, long d) {
		localRegisters[0] = d;
		localRegisters[1] = registers[1];
		localRegisters[2] = registers[2];
		localRegisters[3] = registers[3];
	}
}
