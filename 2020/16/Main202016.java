import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.concurrent.atomic.AtomicInteger;

public class Main202016 {
    AtomicInteger n = new AtomicInteger();

    private void start(String[] args) throws IOException, InterruptedException {
        List<Thread> threads = new ArrayList<>();
        int cores = Runtime.getRuntime().availableProcessors();
        int numberOfThreads = Math.max(cores - 1, 1);
        System.out.format("n: %d\n", numberOfThreads);
        int startIndex = 0;
        numberOfThreads = 1;
        if (args.length > 0) {
            startIndex = Integer.parseInt(args[0]);
        }
        System.out.format("startIndex: %d\n", startIndex);
        for (int i = 0; i < numberOfThreads; i++) {
            threads.add(new Worker(i, startIndex, numberOfThreads, this));
        }
        for (int i = 0; i < numberOfThreads; i++) {
            threads.get(i).start();
        }
        for (int i = 0; i < numberOfThreads; i++) {
            threads.get(i).join();
        }
    }

    public static void main(String[] args) throws IOException, InterruptedException {
        new Main202016().start(args);
    }
}

class Worker extends Thread {

    public Worker(int index, int startIndex, int numberOfThreads, Main202016 parent) {
        this.index = index;
        this.startIndex = startIndex;
        this.numberOfThreads = numberOfThreads;
        this.parent = parent;
    }

    Main202016 parent;
    int index;
    int startIndex;
    int numberOfThreads;

    List<Set<Integer>> rules = new ArrayList<>();
    List<List<Integer>> tickets = new ArrayList<>();
    List<Integer> yourTicket = new ArrayList<>();
    Set<Integer> valids = new HashSet<>();
    Set<Integer> numbersSoFar = new HashSet<>();
    List<Integer> result = new ArrayList<>();
    boolean[][] dp;
    final String FILE_NAME = "16.txt";
    long start;
    long n = 0;

    @Override
    public void run() {
        try {
            foo1();
            foo2();
        } catch(Exception e) {
        }
    }

    private void foo1() throws IOException {
        List<String> lines = Files.readAllLines(Paths.get(FILE_NAME));
        Set<Integer> numbers = new HashSet<>();
        boolean nearby = false;
        long acc = 0;
        for (int i = 0; i < lines.size(); i++) {
            final String line = lines.get(i);
            if (line.startsWith("nearby ticket")) {
                nearby = true;
            }
            if (line.contains("-")) {
                String[] tokens = line.split(" ");
                for (String token: tokens) {
                    if (token.contains("-")) {
                        String[] range = token.split("-");
                        int min = Integer.parseInt(range[0]);
                        int max = Integer.parseInt(range[1]);
                        for (int j = min; j <= max; j++) {
                            numbers.add(j);
                        }
                    }
                }
            }
            if (nearby && line.contains(",")) {
                String[] tokens = line.split(",");
                boolean valid = true;
                for (String token : tokens) {
                    int t = Integer.parseInt(token);
                    if (!numbers.contains(t)) {
                        acc += t;
                        valid = false;
                    }
                }
                if (valid) {
                    valids.add(i);
                }
            }
        }
//        System.out.print("Valid tickets: ");
//        for (int v : valids) {
//            System.out.format("%d ", v);
//        }
//        System.out.println();
        System.out.println(acc);
    }

    private void foo2() throws IOException {
        List<String> lines = Files.readAllLines(Paths.get(FILE_NAME));
        int yourTicketIdx = 0;
        for (int i = 0; i < lines.size(); i++) {
            final String line = lines.get(i);
            if (line.startsWith("your ticket")) {
                yourTicketIdx = i + 1;
                continue;
            }
            if (line.contains("-")) {
                String[] tokens = line.split(" ");
                Set<Integer> numbers = new HashSet<>();
                for (String token: tokens) {
                    if (token.contains("-")) {
                        String[] range = token.split("-");
                        int min = Integer.parseInt(range[0]);
                        int max = Integer.parseInt(range[1]);
                        for (int j = min; j <= max; j++) {
                            numbers.add(j);
                        }
                    }
                }
                rules.add(numbers);
            }
            if (line.contains(",")) {
                if (i != yourTicketIdx && !valids.contains(i)) {
                    continue;
                }
                String[] tokens = line.split(",");
                List<Integer> ticket = new ArrayList<>();
                for (String token : tokens) {
                    int t = Integer.parseInt(token);
                    if (i == yourTicketIdx) {
                        yourTicket.add(t);
                    } else {
                        ticket.add(t);
                    }
                }
                if (i != yourTicketIdx) {
                    tickets.add(ticket);
                }
            }
        }

//        System.out.println("Rules:");
//        for (int i = 0; i < rules.size(); i++) {
//            final Set<Integer> rule = rules.get(i);
//            System.out.print(i + ": ");
//            for (int r : rule) {
//                System.out.format("%d ", r);
//            }
//            System.out.println();
//        }
//        System.out.println();
//        System.out.print("Your ticket: ");
//        for (int v : yourTicket) {
//            System.out.format("%d ", v);
//        }
//        System.out.println();
//        System.out.println("Valid tickets:");
//        for (int i = 0; i < tickets.size(); i++) {
//            final List<Integer> ticket = tickets.get(i);
//            for (int t : ticket) {
//                System.out.format("%3d ", t);
//            }
//            System.out.println();
//        }
//        System.out.println();

        dp = new boolean[rules.size()][rules.size()];
        for (int i = 0; i < rules.size(); i++) {
            for (int j = 0; j < rules.size(); j++) {
                dp[i][j] = isCorrect(i, j);
            }
        }
        for (int i = 0; i < rules.size(); i++) {
            result.add(-1);
        }
        start = System.currentTimeMillis();
        solve(0);
    }

    private void printResult() {
        System.out.print("Result: ");
        for (int r : result) {
            System.out.format("%2d ", r);
        }
        System.out.println();
    }

    private void solve(int idx) {
        if (idx >= result.size()) {
            return;
        }
        parent.n.incrementAndGet();
        int n = parent.n.get();
        if (n % 10000000L == 0) {
            synchronized (parent) {
                System.out.format("{%d} n: %.0f M ", index, n / 1000000.0);
                printResult();
            }
        }
        for (int i = 0; i < result.size(); i++) {
            if (idx == 0 && i < startIndex) {
                continue;
            }
            if (idx == 0 && i % numberOfThreads != index) {
                continue;
            }
            if (numbersSoFar.contains(i)) {
                continue;
            }
            boolean correct = dp[idx][i];
            if (correct) {
                result.set(idx, i);
                if (idx == result.size() - 1) {
                    final long end = System.currentTimeMillis();
                    System.out.println("BINGO");
                    printResult();
                    System.out.format("Elapsed time: %.3f\n", (end - start) / 1000.0);
                    long acc = 1;
                    for (int j = 0; j < yourTicket.size(); j++) {
                        if (result.get(j) < 6) {
                            acc *= yourTicket.get(j);
                        }
                    }
                    System.out.format("n: %d, result: %d\n", n, acc);
                    System.exit(0);
                }
                numbersSoFar.add(i);
                solve(idx + 1);
                numbersSoFar.remove(i);
            }
        }
//        result.set(idx, -1);
    }

    private boolean isCorrect(int idx, int i) {
        for (int t = 0; t < tickets.size(); t++) {
            if (!rules.get(i).contains(tickets.get(t).get(idx))) {
                return false;
            }
        }
        return true;
    }

}
