import java.io.IOException;
import java.math.BigInteger;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

public class Main201605a {
	String key = "wtnhxymk";

	public static void main(String[] args) throws IOException, NoSuchAlgorithmException, InterruptedException {
		Main201605a program = new Main201605a();
		program.one();
		program.two();
	}

	private void one() throws NoSuchAlgorithmException, InterruptedException {
		long start = System.currentTimeMillis();
		MessageDigest md5 = null;
		int n = 0;
		for (int i = 0; i < 8; i++) {
			while (true) {
				n++;
				try {
					md5 = MessageDigest.getInstance("MD5");
				} catch (NoSuchAlgorithmException e) {
					e.printStackTrace();
				}
				String plaintext = key + n;
				md5.update(plaintext.getBytes());
				String hashtext = String.format("%032x", new BigInteger(1, md5.digest()));
				if (hashtext.startsWith("00000")) {
					System.out.print(hashtext.charAt(5));
					break;
				}
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
		MessageDigest md5 = null;
		int n = 0;
		for (int i = 0; i < 8; i++) {
			while (true) {
				n++;
				try {
					md5 = MessageDigest.getInstance("MD5");
				} catch (NoSuchAlgorithmException e) {
					e.printStackTrace();
				}
				String plaintext = key + n;
				md5.update(plaintext.getBytes());
				String hashtext = String.format("%032x", new BigInteger(1, md5.digest()));
				char c = hashtext.charAt(5);
				if (hashtext.startsWith("00000")) {
					if (c >= '0' && c <= '7') {
						if (password[c - '0'] == 0) {
							password[c - '0'] = hashtext.charAt(6);
							System.out.println(password);
							break;
						}
					}
				}
			}
		}
		System.out.println();
		long end = System.currentTimeMillis();
		System.out.println();
		System.out.println((end - start) / 1000 + " sec");
	}
}