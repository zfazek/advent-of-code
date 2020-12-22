package main;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashSet;
import java.util.Set;

class Tile {
	String id;
	String[] lines = new String[10];
	String up;
	String down;
	String left = "";
	String right = "";

	@Override
	public String toString() {
		StringBuilder ret = new StringBuilder();
		ret.append(id).append("\n");
		for (int i = 0; i < 10; i++) {
			ret.append(lines[i]).append("\n");
		}
		ret.append("\n");
		ret.append(up).append("\n");
		ret.append(down).append("\n");
		ret.append(left).append("\n");
		ret.append(right).append("\n");
		ret.append("\n");
		return ret.toString();
	}

	public void rotate() {
		int N = lines.length;
		for (int x = 0; x < N / 2; x++) { 
			for (int y = x; y < N - x - 1; y++) { 
				char temp = lines[x].charAt(y); 
				lines[x] = lines[x].substring(0, y) + lines[y].charAt(N - 1 - x) + lines[x].substring(y + 1); 
				lines[y] = lines[y].substring(0, N - 1 - x) + lines[N - 1 - x].charAt(N - 1 - y) + lines[y].substring(N - x); 
				lines[N - 1 - x] = lines[N - 1 - x].substring(0, N - 1 - y) + lines[N - 1 - y].charAt(x) + lines[N - 1 - x].substring(N - y); 
				lines[N - 1 - y] = lines[N - 1 - y].substring(0, x) + temp + lines[N - 1 - y].substring(x + 1); 
			} 
		} 
	}

	public void flip() {
		for (int i = 0; i < lines.length; i++) {
			StringBuilder str = new StringBuilder();
			str.append(lines[i]);
			lines[i] = str.reverse().toString();
		}
	}
}

public class Main202020 {
	final static String FILE_NAME = "20.txt";
	static Set<Integer> usedIndices = new HashSet<>();
	static long count = 0;

	public static void main(String[] args) throws IOException {
		String file = Files.readString(Paths.get(FILE_NAME));
		String[] tilesStrArr = file.split("\\r\\n\\r\\n");
		Tile[] tiles = new Tile[tilesStrArr.length * 8];
		for (int i = 0; i < tilesStrArr.length * 8; i++) {
			String[] tileStrArr = tilesStrArr[i / 8].split("\\r\\n");
			Tile tile = new Tile();
			tile.id = tileStrArr[0].split(" ")[1].split(":")[0];
			for (int j = 1; j < 11; j++) {
				tile.lines[j - 1] = tileStrArr[j];
			}
			for (int r = 0; r < i % 4; r++) {
				tile.rotate();
			}
			for (int f = 0; f < (i % 8) / 4; f++) {
				tile.flip();
			}
			tile.up = tile.lines[0];
			tile.down = tile.lines[9];
			for (int j = 0; j < 10; j++) {
				tile.left = tile.left + tile.lines[j].charAt(0);
				tile.right = tile.right + tile.lines[j].charAt(9);
			}
			tiles[i] = tile;
		}
		int[] ids = new int[tilesStrArr.length];
		int n = (int)Math.sqrt(tilesStrArr.length);
		iter(0, ids, tiles, n);

	}

	private static void iter(int idx, int[] ids, Tile[] tiles, int n) {
		if (idx >= ids.length) {
			return;
		}
		for (int i = 0; i < tiles.length; i++) {
			count++;
			if (usedIndices.contains(i / 8)) {
				continue;
			}
			ids[idx] = i;
			if (count % 100000000 == 0) {
				System.out.format("count: %d, idx: %d\n", count, idx);
				printIds(ids, tiles);
			}
			boolean good = isGood(idx, ids, tiles, n);
			if (good) {
				if (idx == ids.length - 1) {
					System.out.println("BINGO");
					printIds(ids, tiles);
					long m = Long.parseLong(tiles[ids[0]].id) *
							Long.parseLong(tiles[ids[n - 1]].id) *
							Long.parseLong(tiles[ids[n * n - n]].id) *
							Long.parseLong(tiles[ids[n * n - 1]].id);
					System.out.println(m);
					System.exit(0);
				}
				usedIndices.add(i / 8);
				iter(idx + 1, ids, tiles, n);
				usedIndices.remove(i / 8);
			}
			ids[idx] = 0;
		}
	}

	private static boolean isGood(int idx, int[] ids, Tile[] tiles, int n) {
		if (idx / n > 0) {
			int idx2 = idx - n;
			if (!tiles[ids[idx]].down.equals(tiles[ids[idx2]].up)) {
				return false;
			}
		}
		if (idx % n > 0) {
			int idx2 = idx - 1;
			if (!tiles[ids[idx]].right.equals(tiles[ids[idx2]].left)) {
				return false;
			}
		}
		return true;
	}

	private static void printIds(int[] ids, Tile[] tiles) {
		int n = (int)Math.sqrt(ids.length);
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++) {
				System.out.format("%4s ", tiles[ids[i * n + j]].id);
			}
			System.out.println();
		}
		System.out.println();
	}

}