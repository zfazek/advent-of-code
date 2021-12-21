import java.io.IOException;

public class Main202121 {
	public static void main(String[] args) throws IOException {
		one();
		two();
	}

	private static void one() {
		long start = System.currentTimeMillis();
		Player[] players = new Player[2];
		players[0] = new Player(10);
		players[1] = new Player(4);
		int nextToMoveIdx = 0;
		int t = 0;
		int numOfRolls = 0;
		while (true) {
			t += 3;
			int score = ++numOfRolls + ++numOfRolls + ++numOfRolls;
			int newPos = (players[nextToMoveIdx].pos + score) % 10;
			players[nextToMoveIdx].pos = newPos;
			players[nextToMoveIdx].score += newPos + 1;
			if (players[nextToMoveIdx].score >= 1000) {
				System.out.println(t * players[1 - nextToMoveIdx].score);
				break;
			}
			nextToMoveIdx = 1 - nextToMoveIdx;
		}
		long end = System.currentTimeMillis();
		System.out.format("%d ms\n", end - start);
	}

	private static void two() {

	}
}

class Player {
	int pos;
	int score = 0;

	Player(int pos) {
		this.pos = pos - 1;
	}

	@Override
	public String toString() {
		return String.format("Position: %2d, Score: %4d", pos + 1, score);
	}
}