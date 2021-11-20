import java.io.IOException;
import java.math.BigInteger;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Main201605 {
	String key = "wtnhxymk";
	boolean shouldRun = true;
	int N = 4;
	List<Md5Hasher> workers = new ArrayList<>();
	Queue<Integer> numbers = new LinkedList<>();
	List<String> hashes = new ArrayList<>();
	Input input = new Input();
	Object mutexInputQueue = new Object();
	Object mutexHashes = new Object();

	public static void main(String[] args) throws IOException, NoSuchAlgorithmException, InterruptedException {
		Main201605 program = new Main201605();
		program.start();
		program.one();
		program.two();
		program.stop();
	}

	private void start() {
		input.start();
		for (int i = 0; i < N; i++) {
			Md5Hasher worker = new Md5Hasher();
			workers.add(worker);
			worker.start();
		}
	}

	private void stop() throws InterruptedException {
		input.join();
		for (Md5Hasher worker : workers) {
			worker.join();
		}
	}

	private void one() throws NoSuchAlgorithmException, InterruptedException {
		long start = System.currentTimeMillis();
		String hashtext;
		for (int i = 0; i < 8; i++) {
			while (true) {
				Thread.sleep(1000);
				synchronized (mutexHashes) {
					if (hashes.size() <= i) {
						continue;
					}
					hashtext = hashes.get(i);
				}
				System.out.print(hashtext.charAt(5));
				break;
			}
		}
		System.out.println();
		long end = System.currentTimeMillis();
		System.out.println();
		System.out.println((end - start) / 1000 + " sec");
	}

	private void two() throws NoSuchAlgorithmException, InterruptedException {
		char[] password = new char[8];
		long start = System.currentTimeMillis();
		String hashtext;
		int n = 0;
		for (int i = 0; i < 8; i++) {
			while (true) {
				Thread.sleep(1000);
				synchronized (mutexHashes) {
					if (hashes.size() <= n) {
						continue;
					}
					hashtext = hashes.get(n++);
				}
				char c = hashtext.charAt(5);
				if (c >= '0' && c <= '7') {
					if (password[c - '0'] == 0) {
						password[c - '0'] = hashtext.charAt(6);
						//						System.out.format("i: %d, idx: %c, char: %c\n", i, c, hashtext.charAt(6));
						System.out.println(password);
						break;
					}
				}
			}
		}
		shouldRun = false;
		long end = System.currentTimeMillis();
		System.out.println();
		System.out.println((end - start) / 1000 + " sec");
	}

	class Input extends Thread {

		@Override
		public void run() {
			int n = 0;
			while (shouldRun) {
				synchronized (mutexInputQueue) {
					if (numbers.size() > 100) {
						continue;
					}
					numbers.add(n++);
				}
				if (n == Integer.MAX_VALUE) {
					System.out.println("Input ends");
					break;
				}
			}
		}
	}

	class Md5Hasher extends Thread {

		@Override
		public void run() {
			Integer n = 0;
			MessageDigest md5 = null;
			try {
				md5 = MessageDigest.getInstance("MD5");
			} catch (NoSuchAlgorithmException e) {
				e.printStackTrace();
			}
			while (shouldRun) {
				synchronized (mutexInputQueue) {
					n = numbers.poll();
					if (n == null) {
						continue;
					}
				}
				String plaintext = key + n;
				md5.update(plaintext.getBytes());
				String hashtext =  String.format("%032x", new BigInteger(1, md5.digest()));
				if (hashtext.startsWith("00000")) {
					synchronized (mutexHashes) {
						hashes.add(hashtext);
					}
				}
			}
		}
	}
}