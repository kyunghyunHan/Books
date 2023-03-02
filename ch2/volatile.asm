wait_while_0:
   ldr w8,[x0]    ;
   cbz w8,  .LBB0_2 ;
   ret
.LBB0_2:
   b .LBB0_2