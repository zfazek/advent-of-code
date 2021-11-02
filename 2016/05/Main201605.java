import java.io.IOException;
import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Main201605 {
	String key = "wtnhxymk";
	boolean shouldRun = true;
	int N = 6;
	List<Worker> workers = new ArrayList<>();
	Map<Integer, String> hashes = new HashMap<Integer,String>();
	Object mutex = new Object();

	public static void main(String[] args) throws IOException, NoSuchAlgorithmException, InterruptedException {
		Main201605 program = new Main201605();
		program.start();
		program.one();
		program.two();
		program.stop();
	}

	private void start() {
		for (int i = 0; i < N; i++) {
			Worker worker = new Worker(i, N);
			workers.add(worker);
			worker.start();
		}
	}

	private void stop() throws InterruptedException {
		for (Worker worker : workers) {
			worker.join();
		}
	}

	private void one() throws NoSuchAlgorithmException {
		long start = System.currentTimeMillis();
		int n = 0;
		String hashtext;
		for (int i = 0; i < 8; i++) {
			while (true) {
				synchronized (mutex) {
					if (hashes.containsKey(n))  {
						hashtext = hashes.get(n);
					} else {
						continue;
					}
				}
				if (hashtext.startsWith("00000")) {
					System.out.print(hashtext.charAt(5));
					n++;
					break;
				}
				n++;
			}
		}
		long end = System.currentTimeMillis();
		System.out.println();
		System.out.println((end - start) / 1000 + " sec");
	}

	private void two() throws NoSuchAlgorithmException, InterruptedException {
		char[] password = new char[8];
		long start = System.currentTimeMillis();
		int n = 0;
		String hashtext;
		for (int i = 0; i < 8; i++) {
			while (true) {
				synchronized (mutex) {
					if (hashes.containsKey(n))  {
						hashtext = hashes.get(n);
						hashes.remove(n);
					} else {
						continue;
					}
				}
				n++;
				if (hashtext.startsWith("00000")) {
					char c = hashtext.charAt(5);
					if (c >= '0' && c <= '7') {
						if (password[c - '0'] == 0) {
							password[c - '0'] = hashtext.charAt(6);
							System.out.format("i: %d, n: %d, idx: %c, char: %c\n", i, n, c, hashtext.charAt(6));
							System.out.println(password);
							break;
						}
					}
				}
			}
		}
		shouldRun = false;
		long end = System.currentTimeMillis();
		System.out.println();
		System.out.println((end - start) / 1000 + " sec");
	}

	class Worker extends Thread {

		private int i;
		private int step;

		Worker(int i, int step) {
			this.i = i;
			this.step = step;
		}

		@Override
		public void run() {
			MessageDigest md5 = null;
			try {
				md5 = MessageDigest.getInstance("MD5");
			} catch (NoSuchAlgorithmException e) {
				e.printStackTrace();
			}
			int n = i - step;
			while (shouldRun) {
				n += step;
				String plaintext = key + n;
				md5.update(StandardCharsets.UTF_8.encode(plaintext));
				String hashtext =  String.format("%032x", new BigInteger(1, md5.digest()));
				synchronized (mutex) {
					hashes.put(n, hashtext);
				}
			}
		}
	}
}