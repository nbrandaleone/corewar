export function initReferenceModal() {
  const modal = document.getElementById("ref-modal");
  const body = document.getElementById("ref-body");
  document.getElementById("btn-ref").addEventListener("click", () => modal.showModal());
  document.getElementById("btn-close-ref").addEventListener("click", () => modal.close());

  body.innerHTML = `
    <p><strong>Core War</strong> (D. G. Jones &amp; A. K. Dewdney, 1984) pits assembly programs called <em>warriors</em> against each other inside a circular 8,000-cell MARS virtual memory. A warrior wins when all opponent processes execute <code>DAT</code> or divide by zero.</p>
    <h3>ICWS-94 Opcodes</h3>
    <pre>DAT  data (kills process)       JMP  unconditional jump
MOV  copy A to B                JMZ  jump to A if B is zero
ADD  add A to B                 JMN  jump to A if B != 0
SUB  subtract A from B          DJN  decrement B, jump if != 0
MUL  multiply A by B            SPL  spawn process at A
DIV  divide B by A (0=die)      SEQ  skip next if A == B (alias CMP)
MOD  remainder B % A (0=die)    SNE  skip next if A != B
SLT  skip next if A &lt; B         NOP  no operation</pre>
    <h3>Addressing Modes</h3>
    <pre># immediate   $ direct (default)   * A-indirect   @ B-indirect
{ A-predec    &lt; B-predec           } A-postinc    &gt; B-postinc</pre>
    <h3>Instruction Modifiers</h3>
    <pre>.A  A-field   .B  B-field   .AB  A-&gt;B   .BA  B-&gt;A
.F  both (A-&gt;A, B-&gt;B)   .X  crossed (A-&gt;B, B-&gt;A)   .I  whole instruction</pre>
  `;
}
