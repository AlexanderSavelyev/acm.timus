import java.io.*;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;



public class Task {
   

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

   private boolean buildExperssion(StringBuilder t, int curPos, Pair<Integer, Integer> curUsage) {
      if(t.length() > 20000) {
//         throw new RuntimeException();
         return false;
      }
      int curLength = t.length();
      int curUsageIdx = (curLength - curPos);
      
      String curWord = t.substring(curPos);
      if(wordsContain(curWord) != null) {
         return true;
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
               return false;
            }
            if (buildExperssion(t, curPos + curSubWord.length(), to)) {
               return true;
            }
            usageTree.removeEdge(curUsage, to);
         }
      }
      
      WordList w1 = getAllWordsContainedPrefix(curWord);
      for (Integer curBigIdx : w1.list) {
         String curBigWord = words.get(curBigIdx).substring(curWord.length());
         
//         System.out.println("len = " + curUsageIdx + " w = " + curBigIdx);
         Pair<Integer, Integer> to = new Pair(curUsageIdx, curBigIdx);
         if (!usageTree.addEdge(curUsage, to)) {
            return false;
         }
         t.append(curBigWord);
         if(buildExperssion(t, curLength, to)) {
            return true;
         }
         usageTree.removeEdge(curUsage, to);
         t.setLength(curLength);
      }
      
      return false;
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
      HashMap<String, Integer> hwords = new HashMap<>();
      
      for (int i = 0; i < N; i++) {
         String w = nextString();
         wordMap.put(w, words.size());
         words.add(w);
         wordSizes.add(w.length());
      }
      if(words.size() != wordMap.size()) {
         throw new RuntimeException();
      }
      
      StringBuilder t = new StringBuilder();
      for (int curWord = 0; curWord < words.size(); curWord++) {
         WordList w1 = getAllWordsContainedPrefix(curWord);
         for (Integer curBigWord : w1.list) {
            t.append(words.get(curBigWord));
            int curPos = words.get(curWord).length();
            if(buildExperssion(t, curPos, new Pair(0, curWord))) {
               out.println("YES");
               out.println(t.toString());
               return;
            }
            t.setLength(0);
         }
      }
      
      out.println("NO");
   }
   
   
}