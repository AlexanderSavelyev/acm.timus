/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

import java.io.*;
import org.junit.Test;
import static org.junit.Assert.assertEquals;

/**
 *
 * @author Aleksandr_Savelev
 */
public class TaskTest {

   public TaskTest() {
   }

   @Test
   public void testRun1() throws Exception {
      //Reader reader = new FileReader("input.txt");
      String test = "5\n"
              + "ab\n"
              + "acb\n"
              + "bc\n"
              + "abac\n"
              + "babbc";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
//      Writer writer = new OutputStreamWriter(System.out);
      StringWriter writer = new StringWriter();

      new Task().run(reader, writer);

      assertEquals("YES\nabacbabbc", writer.getBuffer().toString().trim());
   }

   @Test
   public void testRun2() throws Exception {
      //Reader reader = new FileReader("input.txt");
      String test = "2\n"
              + "ab\n"
              + "abab";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
//      Writer writer = new OutputStreamWriter(System.out);
      StringWriter writer = new StringWriter();

      new Task().run(reader, writer);

      assertEquals("YES\nabab", writer.getBuffer().toString().trim());
   }

   @Test
   public void testRun3() throws Exception {
      //Reader reader = new FileReader("input.txt");
      String xx = "ab+aba"
                + "aba+ab";
      String test = "4\n"
              + "ab\n"
              + "ba\n"
              + "aba\n"
              + "bab";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
//      Writer writer = new OutputStreamWriter(System.out);

      StringWriter writer = new StringWriter();

      new Task().run(reader, writer);

      assertEquals("YES\nababa", writer.getBuffer().toString().trim());
   }

   @Test
   public void testRun4() throws Exception {
      //Reader reader = new FileReader("input.txt");
      String test = "3\n"
              + "abcab\n"
              + "abc\n"
              + "c";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
//      Writer writer = new OutputStreamWriter(System.out);
      StringWriter writer = new StringWriter();

      new Task().run(reader, writer);

      assertEquals("YES\nabcabc", writer.getBuffer().toString().trim());
   }

}
