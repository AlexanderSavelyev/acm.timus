
import java.io.*;
import java.util.ArrayList;

public class Task {

   public static void main(String[] args) throws IOException {
      Reader reader = new InputStreamReader(System.in);
      Writer writer = new OutputStreamWriter(System.out);
      new Task().run(reader, writer);
   }
   StreamTokenizer in;
   PrintWriter out;

   int nextInt() throws IOException {
      in.nextToken();
      return (int) in.nval;
   }

   void run(Reader reader, Writer writer) throws IOException {
      in = new StreamTokenizer(new BufferedReader(reader));
      out = new PrintWriter(writer);
      solve();
      out.flush();
   }

   private double dist(Circle c1, Circle c2) {
      return Math.sqrt(Math.pow(c1.center.first - c2.center.first, 2) + Math.pow(c1.center.second - c2.center.second, 2));
   }

   public class Pair<A, B> {

      public A first;
      public B second;

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
            Pair otherPair = (Pair) other;
            return ((this.first == otherPair.first
                    || (this.first != null && otherPair.first != null
                    && this.first.equals(otherPair.first)))
                    && (this.second == otherPair.second
                    || (this.second != null && otherPair.second != null
                    && this.second.equals(otherPair.second))));
      }

      public String toString() {
         return "(" + first + ", " + second + ")";
      }

   }

   class Circle {

      public Pair<Integer, Integer> center;
      public int radius;

      private Circle(int x, int y, int r) {
         center = new Pair<>(x, y);
         radius = r;
      }

   }

   void solve() throws IOException {
      int N = nextInt();
      ArrayList<Circle> circles = new ArrayList<>(N);
      for (int i = 0; i < N; i++) {
         Circle c = new Circle(nextInt(), nextInt(), nextInt());
         for(int j = 0; j < i; j++) {
            Circle c2 = circles.get(j);
            double d = dist(c, c2);
            if(d < (c.radius + c2.radius)) {
               
            }
         }
         circles.add(c);
      }
      out.println(1);
   }
   

}
