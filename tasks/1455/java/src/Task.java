import java.io.*;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;



public class Task {

   private String result;


   public class Pair<A, B> {
      public A first;
      public B second;
      public Pair(A first, B second) {
         super();
         this.first = first;
         this.second = second;
      }
      @Override
      public int hashCode() {
         int hashFirst = first != null ? first.hashCode() : 0;
         int hashSecond = second != null ? second.hashCode() : 0;
         return (hashFirst + hashSecond) * hashSecond + hashFirst;
      }
      @Override
      public boolean equals(Object other) {
         Pair otherPair = (Pair) other;
         return ((this.first == otherPair.first
                 || (this.first != null && otherPair.first != null
                 && this.first.equals(otherPair.first)))
                 && (this.second == otherPair.second
                 || (this.second != null && otherPair.second != null
                 && this.second.equals(otherPair.second))));
      }
      @Override
      public String toString() {
         return "(" + first + ", " + second + ")";
      }
   }
   
   class UsageTree {
      public HashMap<Pair<Integer, Integer>, HashSet<Pair<Integer, Integer>>> adj = new HashMap<>();
      public boolean addEdge(Pair<Integer, Integer> from, Pair<Integer, Integer> to) {
         if(!adj.containsKey(from)) {
            adj.put(from, new HashSet<>());
         }
         if(adj.get(from).contains(to)) {
            return false;
         }
         adj.get(from).add(to);
         return true;
      }
      public void removeEdge(Pair<Integer, Integer> from, Pair<Integer, Integer> to) {
         adj.get(from).remove(to);
      }
      public String toString() {
         StringBuilder res = new StringBuilder();
         for(Pair<Integer, Integer> k:adj.keySet()) {
            res.append("Key = ").append(k).append("\n");
            for(Pair<Integer, Integer> v:adj.get(k)) {
               res.append("Value = ").append(v.toString()).append("\n");
            }
            res.append("\n\n\n");
         }
         return res.toString();
      }
   }
   
   
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
   String nextString() throws IOException {
      in.nextToken();
      return in.sval;
   }

   void run(Reader reader, Writer writer) throws IOException {
      in = new StreamTokenizer(new BufferedReader(reader));
      out = new PrintWriter(writer);
      solve();
      out.flush();
   }
   
   class WordList {
      public LinkedList<Integer> list = new LinkedList<>();
   }
   
   ArrayList<String> words = new ArrayList<>();
   HashMap<String, Integer> wordMap = new HashMap<>();
   HashSet<Integer> wordSizes = new HashSet<>();
   HashMap<String, WordList> prefixes = new HashMap<>();
   HashSet<Integer> calculatedPrefixes = new HashSet<>();
   UsageTree usageTree = new UsageTree();

   private void buildExperssion(StringBuilder t, int curPos, Pair<Integer, Integer> curUsage) {
      if(t.length() > 200000) {
         throw new RuntimeException();
//         return false;
      }
      if(result != null && t.length() >= result.length()) {
         return;
      }
      int curLength = t.length();
      int curUsageIdx = (curLength - curPos);
      
      String curWord = t.substring(curPos);
      if(wordsContain(curWord) != null) {
         result = t.toString();
         return;
      }
      
      for (int subWordSize = 1; subWordSize < curWord.length(); subWordSize++) {
         if(!sizeExists(subWordSize)) {
            continue;
         }
         String curSubWord = curWord.substring(0, subWordSize);
         Integer exWord = wordsContain(curSubWord);
         if (exWord != null) {
            Pair<Integer, Integer> to = new Pair(curUsageIdx, exWord);
            if (!usageTree.addEdge(curUsage, to)) {
               return;
            }
            buildExperssion(t, curPos + curSubWord.length(), to);
            usageTree.removeEdge(curUsage, to);
         }
      }
      
      WordList w1 = getAllWordsContainedPrefix(curWord);
      for (Integer curBigIdx : w1.list) {
         String curBigWord = words.get(curBigIdx).substring(curWord.length());
         
//         System.out.println("len = " + curUsageIdx + " w = " + curBigIdx);
         Pair<Integer, Integer> to = new Pair(curUsageIdx, curBigIdx);
         if (!usageTree.addEdge(curUsage, to)) {
            return;
         }
         t.append(curBigWord);
         buildExperssion(t, curLength, to);
         usageTree.removeEdge(curUsage, to);
         t.setLength(curLength);
      }
   }

   private WordList getAllWordsContainedPrefix(int w) {
      return getAllWordsContainedPrefix(words.get(w));
   }
   
   private Integer wordsContain(String tStr) {
      return wordMap.get(tStr);
   }

   private WordList getAllWordsContainedPrefix(String prefix) {
      int curLen = prefix.length();
      if(!calculatedPrefixes.contains(curLen)) {
         for (int curWord = 0; curWord < words.size(); curWord++) {
            String word = words.get(curWord);
            if(word.length() <= curLen) {
               continue;
            }
            String subWord = word.substring(0, curLen);
            if(!prefixes.containsKey(subWord)) {
               prefixes.put(subWord, new WordList());
            }
            WordList wordList = prefixes.get(subWord);
            wordList.list.add(curWord);
         }
         calculatedPrefixes.add(curLen);
      }
      if(prefixes.containsKey(prefix)) {
         return prefixes.get(prefix);
      }
      
      return new WordList();
   }

   private boolean sizeExists(int subWordSize) {
      return wordSizes.contains(subWordSize);
   }
   
   
   void solve() throws IOException {
      int N = nextInt();
      
      for (int i = 0; i < N; i++) {
         String w = nextString();
         wordMap.put(w, words.size());
         words.add(w);
         wordSizes.add(w.length());
      }
      if(words.size() != wordMap.size()) {
         throw new RuntimeException();
      }
      result = null;
      StringBuilder t = new StringBuilder();
      for (int curWord = 0; curWord < words.size(); curWord++) {
         WordList w1 = getAllWordsContainedPrefix(curWord);
         for (Integer curBigWord : w1.list) {
            t.append(words.get(curBigWord));
            int curPos = words.get(curWord).length();
            buildExperssion(t, curPos, new Pair(-1, curWord));
            if(result != null){
//               if(t.length() > 19000) {
//                  throw new RuntimeException();
//                  t.setLength(20000);
//               }
               out.println("YES");
               out.println(result);
               return;
            }
            t.setLength(0);
         }
      }
      
      out.println("NO");
   }
   
   
}