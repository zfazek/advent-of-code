
import java.io.IOException;
import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

public class Main201504 {

	public static void main(String[] args) throws IOException, NoSuchAlgorithmException {
		MessageDigest md5 = MessageDigest.getInstance("MD5");
		String key = "iwrupvqb";
		long n = 0;
		boolean flag = false;
		while (true) {
			String plaintext = key + n;
			md5.update(StandardCharsets.UTF_8.encode(plaintext));
			String hashtext =  String.format("%032x", new BigInteger(1, md5.digest()));
			if (!flag && hashtext.startsWith("00000")) {
				System.out.println(n);
				flag = true;
			}
			if (hashtext.startsWith("000000")) {
				System.out.println(n);
				break;
			}
			n++;
		}
	}
}

