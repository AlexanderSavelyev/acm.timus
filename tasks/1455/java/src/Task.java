import java.io.*;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;



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
   HashSet<String> wordSet = new HashSet<>();
   HashSet<Integer> wordSizes = new HashSet<>();
   HashMap<String, WordList> prefixes = new HashMap<>();
   HashSet<Integer> calculatedPrefixes = new HashSet<>();
   

   private boolean buildExperssion(StringBuilder t, int curPos) {
      if(t.length() > 20000) {
         return false;
      }
      String curWord = t.substring(curPos);
      if(wordsContains(curWord)) {
         return true;
      }
      
      for (int subWordSize = 1; subWordSize < curWord.length() && sizeExists(subWordSize); subWordSize++) {
         String curSubWord = curWord.substring(0, subWordSize);
         if(wordsContains(curSubWord)) {
            if(buildExperssion(t, curPos + curSubWord.length())) {
               return true;
            }
         }
      }
      
      WordList w1 = getAllWordsContainedPrefix(curWord);
      for (Integer curBigIdx : w1.list) {
         String curBigWord = words.get(curBigIdx).substring(curWord.length());
         int curLength = t.length();
         t.append(curBigWord);
         if(buildExperssion(t, curPos + words.get(curBigIdx).length())) {
            return true;
         }
         t.setLength(curLength);
      }
      
      return false;
   }

   private WordList getAllWordsContainedPrefix(int w) {
      return getAllWordsContainedPrefix(words.get(w));
   }
   
   private boolean wordsContains(String tStr) {
      return wordSet.contains(tStr);
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
         words.add(w);
         wordSet.add(w);
         wordSizes.add(w.length());
      }
      
      StringBuilder t = new StringBuilder();
      for (int curWord = 0; curWord < words.size(); curWord++) {
         WordList w1 = getAllWordsContainedPrefix(curWord);
         for (Integer curBigWord : w1.list) {
            t.append(words.get(curBigWord));
            int curPos = words.get(curBigWord).length() - words.get(curWord).length();
            if(buildExperssion(t, curPos)) {
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