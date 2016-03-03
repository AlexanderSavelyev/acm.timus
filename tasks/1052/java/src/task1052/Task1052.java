/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package task1052;

import java.io.*;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;

public class Task1052 {

    StreamTokenizer in;
    PrintWriter out;
    HashMap<Pair<Float, Float>, HashSet<Pair<Integer, Integer>>> lines = new HashMap<>();
    int res = 0;

    int nextInt() throws IOException {
        in.nextToken();
        return (int) in.nval;
    }

    public static void main(String[] args) throws IOException {
        new Task1052().run();
    }

    void run() throws IOException {
        boolean oj = System.getProperty("ONLINE_JUDGE") != null;
        Reader reader = oj ? new InputStreamReader(System.in) : new FileReader("input.txt");
//Writer writer = oj ? new OutputStreamWriter(System.out) : new FileWriter("output.txt");
        Writer writer = new OutputStreamWriter(System.out);
        in = new StreamTokenizer(new BufferedReader(reader));
        out = new PrintWriter(writer);
        solve();
        out.flush();
    }

    void solve() throws IOException {
        int N = nextInt();
        ArrayList<Pair<Integer, Integer>> coordinates = new ArrayList<>();
        for (int i = 0; i < N; i++) {
            coordinates.add(new Pair(nextInt(), nextInt()));
        }
        HashSet<Pair<Integer, Integer>> xAxis = new HashSet<>();
        HashSet<Pair<Integer, Integer>> yAxis = new HashSet<>();

        for (int i = 0; i < N - 1; i++) {
            for (int j = i + 1; j < N; j++) {
                Pair<Integer, Integer> p1 = coordinates.get(i);
                Pair<Integer, Integer> p2 = coordinates.get(j);
                int x1 = p1.getFirst();
                int y1 = p1.getSecond();
                int x2 = p2.getFirst();
                int y2 = p2.getSecond();
                if (x1 == x2) {
                    if (x1 == 0) {
                        xAxis.add(p1);
                        xAxis.add(p2);
                        if (xAxis.size() > res) {
                            res = xAxis.size();
                        }
                    } else {
                        addLine(1.0f / (float) x1, 0, p1, p2);
                    }
                } else if (y1 == y2) {
                    if (y1 == 0) {
                        yAxis.add(p1);
                        yAxis.add(p2);
                        if (yAxis.size() > res) {
                            res = yAxis.size();
                        }
                    } else {
                        addLine(0, 1.0f / (float) y1, p1, p2);
                    }
                } else {
                    float la = (float) (y2 - y1) / (float) (x1 - x2);
                    float b = 1.0f / (float) (la * x1 + y1);
                    addLine(b * la, b, p1, p2);
                }
            }
        }
        out.println(res);
    }

    private void addLine(float a, float b, Pair<Integer, Integer> p1, Pair<Integer, Integer> p2) {
        Pair<Float, Float> k = new Pair<>(a, b);
        HashSet<Pair<Integer, Integer>> v = lines.get(k);
        if (v == null) {
            v = new HashSet<>();
            lines.put(k, v);
        }

        v.add(p1);
        v.add(p2);

        if (v.size() > res) {
            res = v.size();
        }
    }

    public class Pair<A, B> {

        private A first;
        private B second;

        public Pair(A first, B second) {
            super();
            this.first = first;
            this.second = second;
        }

        public int hashCode() {
            int hashFirst = first != null ? first.hashCode() : 0;
            int hashSecond = second != null ? second.hashCode() : 0;

            return (hashFirst + hashSecond) * hashSecond + hashFirst;
        }

        public boolean equals(Object other) {
            if (other instanceof Pair) {
                Pair otherPair = (Pair) other;
                return ((this.first == otherPair.first
                        || (this.first != null && otherPair.first != null
                        && this.first.equals(otherPair.first)))
                        && (this.second == otherPair.second
                        || (this.second != null && otherPair.second != null
                        && this.second.equals(otherPair.second))));
            }

            return false;
        }

        public String toString() {
            return "(" + first + ", " + second + ")";
        }

        public A getFirst() {
            return first;
        }

        public void setFirst(A first) {
            this.first = first;
        }

        public B getSecond() {
            return second;
        }

        public void setSecond(B second) {
            this.second = second;
        }
    }

}
