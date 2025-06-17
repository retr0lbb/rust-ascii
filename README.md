# Rust Image to ASCII converter

this little and funny rust project can pickup any image in your computer and transform the image in simple ASCII image.


### Use Syntax
cargo run [image path] [scale of reduction base=6] [is colored?]

``cargo run saint-91 10 colored``

```MWWWMMWWWMWWWMBMWMMMMMMMMWWMMWMWWWMMMMMMWWMMMWMWWMWMMWWMWWWMWMWWMWMWMMWMWWMMMBWMWWWMMM
MWMMMWWMWWMWMBMWBMWWWMWMWMMMWMBMWMWWMWMBMWMMBWWWWMBBBBWMWMMWWWMMWMMMWMMBBMWWWBMWMBWMMM
WWMWMBMWMMWMMMBMWWBMMBMWMMMWMWWMBMBWBMWMWWBMWMBBMMMWMWMBMMBBBMMMMMMWMWWWMMBMWWBMMMMMBW
MMMWWWMWMBBMMWWWMWMMMWWMMMWWWWMBMBMMBWBMMMWMWBMMMWMBBMWMMMWMMBBWMMMWMWWWMMMBMMMMWWBMMM
MWMMWMMWWBMWMMWMWWWWMWWMWMMMBMBWBBWBWM .:       BMWMMMBMMMBWMMBWMMBMWMMMBMWMMWMMWWWWMW
BWMMWWWWWWMBMWMMMWMBMMMBMMMWMMMMWWW@ BMWMWBMWMBM!  MWBMBWMMBMWWMMMWBMMMMWWBMMWMMWBWWWW
WWBMWMWBBWWMMMWWMMBMWBMMMBWMBWWWBW  MBBWMWWBBX:; MBW MWWWBMBMMBBMBMBBMMWBMWMMBWBMWWWMW
WBBMWMMBBMBMMMBMWMWBMMWMMMMWMBMBW  BMMM  MMM@    WMW WBWBWMMWBMMBMMWMMMWWBMMWMMMBWMWMW
WBBWMWMBWWMWWWBWMWMM:   : BBBMWB MM BWB:::MMBMMMWMW B:BWMWBWM     :MMWMMMMMWBBMMWMMMMM
WMWWMMWMMMMMWMWBBM oWMMMW .MMMMMW M.M MWMWBWWWBBWMM B:BMBWWW.WWMBWB WMBMMMMBMBBMWMWMMM
MWMBBMMWBMMMWMWMM BMM.. MM MWBWBB:    .   :   .  .   MMMMWM MMM .MMM WWWMBMMWWMBMWMWMM
MMWMWMWMMMWWBWWW MBM MB:WM  MWMMM:::.::  :.     .::  WWWMMM:MM BB:@WB.BMMMMBMBMMMMBMBM
MWBMWMWMMMMBMWM BM@ WWBW.BM:MBBM M  MWWMWMMBB BBMM:W MWMMM BBX.MBB:@WB MMBMWWWMWMBWWBB
BMMMMWMMWMMWMW @WM MB:BM  WW:MMWW MM MMMMMW :.BBMW @:BWMM:MBB WB MM @WM BBMWWBBWWWBMWW
WMMBWMBWMMBMM@ MM:BM MMMW .WM. BB@:B  WMMW.   BWW.W BWMB MWW BMBM @M BW. BWMMWBMWBMBWM
MMBMMMWMWBMMW WW.@  @BB BMM MBM:@B M B::MWMBBMBx W.WMB. MM  MW MBM  H MW MWWMMMWBBWMWB
WMMWWMWWWBMM@ MW BW M@ BBM B  MMM  B WMW  @BWB W MMB  MMM:WW MM: M MB MM  MMMMMWWMWMMM
MMWMMBMMMWMB:BM:WMM  W W  MWBM .BB   @MBMWMMMMBM  .MMMW BMWW: B:@. MMB WM MWBMMWBMWBWM
MWBWBMWBMWMB WM WWB MB  B:W WM  *M WBBMMMB   MWB@MW.M::!WB:WX   BM @BB WW WWMWWMBBWMWB
MWBBBBMBMWWM W@:MW WMW BM   @MWWBBB  MMWM .B B:WM.:MMBMBB    MM MMM MM ~M @WMMWWMMMMBB
MMMBBBBWMBM; M.@M  MMW MM WMMMBWM WWMM :  W   M @BWB@WWMBMM  WM; MM  WB M  WBMMMBWWMMB
WWWMWWBWMMB  W:M:B WM WM MBWMWMWBB BWBW @@ M M @xWBB MMMWBWMW MB BB M M MM BMWWMMWBMMM
MBMMMMMMMBM  M WMM.M  @.WBMMBM BBBMM MW! W@M *@W MM@@MWB WMMB.WM  M MMM M@ MMWWBBWWBBM
MBWMWMWBMMW  W WWW.@MW :MWBMB@B BBWMMM:@MMM @0BB MW WWMM BBMMW  MWB MWM Bx.MWWBBWWWWMB
MMWBWWMMMWM@ M MBB BBM:MMBBMB:B.WW:WMM WMMMM:MMM M  MMBM M BMMM:MWW WMB W  MBMWMMMBWMM
BBMWMWWWWBMW:M:MBM.MMB WMMWWB 0 MW WMM MMBMM.BMW @ MMMB  W:BMMM BMM MMB B BMMBWMMMMMMM
WBMBMWMMWMMB W:BBM BM MMBWM MM  :MB W MBBMBM.WWMW.MMWWM:B WBBMMM.WM WMW W MMBWBWWWMWMW
MMMMMMBBWBWW@.:MM xBB WWWMWW  M. :  BWMBWM. :WMWMMM   Q  BWMBMWW BMW MM   WWMWMBMWMMBM
WWBBMMMMMBMMW  MW.MW:WMM   @ :BM.WM MMMM ,MWBB BBWW WW OW: MB  WW.MM:WW: MMBBMMBWWMWWM
MBWMMWWMMWMMM :M MMB MBWBB. WBB.BBM.MM@ MMBW MW  WM@ MM.MMBB WWMB WBM.B  WMMWWMWMBMMMB
WWMWWMBMMWMWM.: MBMM WMMM.MMMMM:BMB W M BMMM.MW M M@ BM @MMBM:WMW:WMBM.  BMMMMBMMBMWMW
MWMBWBWMWWMMWWMMWWBB:MBBB=MMBM.MMMB MMW:MMB MMM WBM  MBW BMMWBBWM BWWMMBWWBWWMWMWBMMBW
MMMWMMMMMWMWMMMBBMMMW BMWWMWBM:BBWB:WMW.BM MBMM BWW% MBM WBMMWWW:MBWMMBMMWMMWBBWBMMMWW
WMMBWBBMMBWBWBBMWWMMWMMBBMBBWMMMMWMMWMWBMMBWBBBMWBMWWWBWMMWMBBMMMMMMMBWWMMBWMWMWMWMMMW
MWMWBBWMWBMMWMWWBBB*,MMMWMW@WMMMMWWWM@WWWM@WW@WWW@@@WBWWBWBB !BMWWWB@MBWBMBMMBMWMBMMMM
MMMMMBBBMWBWMWMMM  WB: WMM: . WMW  MM  .WM :BMMW. BMMBMMMM  WW :W   .WBMBWBWWBMMMWMWBM
WMMMWWWWMMMMMWWMMM. : WWMW B@ BBW :MB  . M: BBBW :WMBWWMMM  MM. MBW  MMWBMWMWMMBBWWMMW
MMMWMBMBBMWWBWBBMMBBW0 WM...:. MW. MM: B%   BMBB  WMWWWWWBMBWM  BMB  MMBWMBMMBMBMMMBWM
BMWWMWWWBWWMMWBMM.  :::M  BBMB: M :MW :WMM .MBWM  WBBMMBWM*: .:MWMW.:WMMWWMBWMWBBMWMMW
MWMMWWMMWMWMMMBMBMBWBBMMMWMMBWWWMMBWWMMMWMBWMWMMMWBMWWMMBWMWWWWMWMBMBWWWMBWMMBMMBWMBBW
MMWMWWWMBBBMMMWBMBBMWWMBWMBWWWMMMBBWMWWMMWMMMMMWWWBWWWWMBBMMWMMMBMMWWMWMMMMMMMWBWWWWMM
MMWMMWWWWBBWMBWMMWMMMMMMBMMMWMBMBMMWMMMWMWMWMMWMWMMMBBMMMBMMMWBWMBMMMWMMWMMWWMBBWMMBMM
MBMBBWWMMBWWBBMMWWMMBWMMMMMWBMMMBWWMMMMMWMWMWMMBWBMMMMBMBBBWWMMMBMMMMMBMWWBWWBBBBMWMMM



cargo run gene.png 5




                                            ~=Q$!
                     .@M                 ,%%QQ%QQQ!
                    WOW@8               Q%Q%%%%@H@W@$ W@@@
                    H@@WWW             Q%QQQ-@@@@@@@@W@x@W@
                    M%@@@W@.          !%QQ%W@@@WWW@W@WWW@WW^ Q%Q%Q%
                      BW@W@@W=        !%Q%$WW@Wo@WWMW@WWW@W@QQQ%QQ%%
                   W@@@@@@WWW@W        0%MQ,@MBHo;;@@WW@@@W=QQ%QQ%QQ%
                   @&-,W@@WW@@W@W        @@@:,;W@W@W@WW@@W@@W$%%QQQ%Q
                         @WW@W@@@@x       W@@W@WW@@W@@@@WWW@W@QQQQ%%;
                           W@W@WWWWW;    W@@WM$@WWWW.,,;,MQQQ%Q%Q%%^
                             MWWWW@@$&O BW@W@$%Q#,;WWW@WW@W;QQ%Q%+
                               ~W@W@BQXQ 0@W@B@@W@W@@@W%;
                              0WW@@=@WWQQ% %::W0:W@@M@@@
                                   &HW%%%Q#Q%;@W@;,W@
                                   8QQQ%Q0Q%0Q%Q%QQ
                                   ^QW@Q%%Q^o%%%%Q#~
                                   ,@W@H,=%%H%Q%%%QQQ%%!
                                   ,,8Q,;.~M@M%Q%%%Q%Q%Q@%$~
                                    ,;,,;,WH@%%XH%QQQWQWB%%W@WM^
                                    !,;,,,WWWoQMMWMQQ%BB!@@@@@WWWW@:
                                     o;,;;WW@@W@@W;;;;,,+@WWWWW@@@WWWW@@;
                                        ;;#@WM@@W@.;;    ;W+,O@@@WW@@W@@W@@@@O^
                                          ^W@@@W@WW@@;QQQ%X%o%Q%MW@@W@@WW@WW@@W@@W;
                                           W@@WW@WWW@W@@,      ,Q%,HWWWWWW@WxXWW+^
                                           W@W#8WW@@W@WW@@@      %Q! oWWW ^@W@H-
                                          :WWWWWW@@W@@@W@WW@@#   0QQ  &*
                                           ;@W@@@!   MWWWW@WW@@- QQQ
                                            @W@@WM      @WWWWWWW@$Q,^
                                            W@@WW@        ;@@WW@@@W+H@;
                              O*+*^         .@W@WWo          WWW@@@WWWW;
                       -%Q%Q%Q$##$%Q%QQQQQ%Qx@@@@@x     -,QQ%Q%^WW@WWWWW@W,
                     Q%;                   ,;B@@WW@QQ%%QQQQQ;     @WW@@W@@W@W;
                   *%                         @WWWW-                HWWW@W@@@@W&
                   #!                         @@@@WW  +               Q@@WWWW@WW@.
                   =!        ^#     !         @W@@@WMWW                 W@@@W@@@@WM
                                      -       oWW@WWWWM                   @W@@W@WW@0
                       ,             ;        .WW@WWW@*                    ;WWW@@@W@
                           !,;,;;!            ;WWWWW@W@                      W@WWQW@
                                              ,W@@@@WWW$                      WM@WW
                                              :W@WW@W@@- &X
                                              *WWWW@@@W:
                                              WW@W@WW@@!
                                             ;W@WWWWWWW
                                             @&@WxWW@@,
                                             @@WWWWWW


```
