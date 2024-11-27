macro_rules! xterm_colors {
    (
        $enum_name:ident {
            $($variant_name:ident, $xterm_name:expr, $color_number:expr, ($r:expr, $g:expr, $b:expr));* $(;)?
        }
    ) => {
        ///List of ANSI256 colors by name
        pub enum $enum_name {
            $(
                $variant_name,
            )*
        }

        impl $enum_name {
            ///Get all values of variant
            pub fn get_color(&'_ self) -> (&'_ str, u8, [u8;3]) {
                match self {
                    $(
                    Self::$variant_name => ($xterm_name, $color_number, [$r, $g, $b]),
                    )*
                }
            }
            ///Get rgb value of variant
            pub fn rgb(&self) -> [u8;3] {
                match self {
                    $(
                        $enum_name::$variant_name=> [$r, $g, $b],
                    )*
                }
            }
            ///Get ansi256 code of variant
            pub fn ansi256(&self) -> u8 {
                match self {
                    $(
                        $enum_name::$variant_name => $color_number,
                    )*
                }
            }
            ///Get xterm name of variant
            pub fn xterm_name(&self) -> &'static str {
                match self {
                    $(
                        $enum_name::$variant_name => $xterm_name,
                    )*
                }
            }
            ///Get variant by Xterm name
            pub fn get_name(name: &str) -> Option<Self> {
                match name {
                    $(
                        $xterm_name => Some(Self::$variant_name),
                    )*
                    _ => None
                }
            }
            ///Get variant by ansi256 code
            pub fn get_ansi256(ansi256: u8) -> Self {
                match ansi256 {
                    $(
                        $color_number => Self::$variant_name,
                    )*
                }
            }
        }
        // Add utility methods to the enum
    };
}

xterm_colors! {
    XtermColors {
        Black ,  "Black" ,  0 ,  (0 , 0 , 0);
        Maroon ,  "Maroon" ,  1 ,  (128 , 0 , 0);
        Green ,  "Green" ,  2 ,  (0 , 128 , 0);
        Olive ,  "Olive" ,  3 ,  (128 , 128 , 0);
        Navy ,  "Navy" ,  4 ,  (0 , 0 , 128);
        Purple ,  "Purple" ,  5 ,  (128 , 0 , 128);
        Teal ,  "Teal" ,  6 ,  (0 , 128 , 128);
        Silver ,  "Silver" ,  7 ,  (192 , 192 , 192);
        Grey ,  "Grey" ,  8 ,  (128 , 128 , 128);
        Red ,  "Red" ,  9 ,  (255 , 0 , 0);
        Lime ,  "Lime" ,  10 ,  (0 , 255 , 0);
        Yellow ,  "Yellow" ,  11 ,  (255 , 255 , 0);
        Blue ,  "Blue" ,  12 ,  (0 , 0 , 255);
        Fuchsia ,  "Fuchsia" ,  13 ,  (255 , 0 , 255);
        Aqua ,  "Aqua" ,  14 ,  (0 , 255 , 255);
        White ,  "White" ,  15 ,  (255 , 255 , 255);
        Grey0 ,  "Grey0" ,  16 ,  (0 , 0 , 0);
        DarkBlue ,  "DarkBlue" ,  17 ,  (0 , 0 , 95);
        DeepBlue ,  "DeepBlue" ,  18 ,  (0 , 0 , 135);
        RoyalBlue ,  "RoyalBlue" ,  19 ,  (0 , 0 , 175);
        PureBlue ,  "PureBlue" ,  20 ,  (0 , 0 , 215);
        PrimaryBlue ,  "PrimaryBlue" ,  21 ,  (0 , 0 , 255);
        DeepGreen ,  "DeepGreen" ,  22 ,  (0 , 95 , 0);
        DarkTurquoise ,  "DarkTurquoise" ,  23 ,  (0 , 95 , 95);
        DeepSeaBlue ,  "DeepSeaBlue" ,  24 ,  (0 , 95 , 135);
        OceanBlue ,  "OceanBlue" ,  25 ,  (0 , 95 , 175);
        CeruleanBlue ,  "CeruleanBlue" ,  26 ,  (0 , 95 , 215);
        BrightBlue ,  "BrightBlue" ,  27 ,  (0 , 95 , 255);
        TrueGreen ,  "TrueGreen" ,  28 ,  (0 , 135 , 0);
        DarkSeaGreen ,  "DarkSeaGreen" ,  29 ,  (0 , 135 , 95);
        DarkCyan ,  "DarkCyan" ,  30 ,  (0 , 135 , 135);
        TealBlue ,  "TealBlue" ,  31 ,  (0 , 135 , 175);
        Cerulean ,  "Cerulean" ,  32 ,  (0 , 135 , 215);
        Azure ,  "Azure" ,  33 ,  (0 , 135 , 255);
        TrueGreen2 ,  "TrueGreen2" ,  34 ,  (0 , 175 , 0);
        Shamrock ,  "Shamrock" ,  35 ,  (0 , 175 , 95);
        GreenBlue ,  "GreenBlue" ,  36 ,  (0 , 175 , 135);
        Turquoise ,  "Turquoise" ,  37 ,  (0 , 175 , 175);
        TurquoiseBlue ,  "TurquoiseBlue" ,  38 ,  (0 , 175 , 215);
        Azure2 ,  "Azure2" ,  39 ,  (0 , 175 , 255);
        VibrantGreen ,  "VibrantGreen" ,  40 ,  (0 , 215 , 0);
        TealishGreen ,  "TealishGreen" ,  41 ,  (0 , 215 , 95);
        AquaGreen ,  "AquaGreen" ,  42 ,  (0 , 215 , 135);
        Aquamarine ,  "Aquamarine" ,  43 ,  (0 , 215 , 175);
        AquaBlue ,  "AquaBlue" ,  44 ,  (0 , 215 , 215);
        NeonBlue ,  "NeonBlue" ,  45 ,  (0 , 215 , 255);
        BrightGreen ,  "BrightGreen" ,  46 ,  (0 , 255 , 0);
        MintyGreen ,  "MintyGreen" ,  47 ,  (0 , 255 , 95);
        TurquoiseGreen ,  "TurquoiseGreen" ,  48 ,  (0 , 255 , 135);
        GreenishTurquoise ,  "GreenishTurquoise" ,  49 ,  (0 , 255 , 175);
        BrightTeal ,  "BrightTeal" ,  50 ,  (0 , 255 , 215);
        Cyan ,  "Cyan" ,  51 ,  (0 , 255 , 255);
        DriedBlood ,  "DriedBlood" ,  52 ,  (95 , 0 , 0);
        RichPurple ,  "RichPurple" ,  53 ,  (95 , 0 , 95);
        RoyalPurple ,  "RoyalPurple" ,  54 ,  (95 , 0 , 135);
        VioletBlue ,  "VioletBlue" ,  55 ,  (95 , 0 , 175);
        BlueViolet ,  "BlueViolet" ,  56 ,  (95 , 0 , 215);
        BlueViolet2 ,  "BlueViolet2" ,  57 ,  (95 , 0 , 255);
        MudGreen ,  "MudGreen" ,  58 ,  (95 , 95 , 0);
        Grey37 ,  "Grey37" ,  59 ,  (95 , 95 , 95);
        Dusk ,  "Dusk" ,  60 ,  (95 , 95 , 135);
        Iris ,  "Iris" ,  61 ,  (95 , 95 , 175);
        DarkPeriwinkle ,  "DarkPeriwinkle" ,  62 ,  (95 , 95 , 215);
        Cornflower ,  "Cornflower" ,  63 ,  (95 , 95 , 255);
        OliveGreen ,  "OliveGreen" ,  64 ,  (95 , 135 , 0);
        DarkSage ,  "DarkSage" ,  65 ,  (95 , 135 , 95);
        BlueGrey ,  "BlueGrey" ,  66 ,  (95 , 135 , 135);
        DustyBlue ,  "DustyBlue" ,  67 ,  (95 , 135 , 175);
        SoftBlue ,  "SoftBlue" ,  68 ,  (95 , 135 , 215);
        Cornflower2 ,  "Cornflower2" ,  69 ,  (95 , 135 , 255);
        KermitGreen ,  "KermitGreen" ,  70 ,  (95 , 175 , 0);
        BoringGreen ,  "BoringGreen" ,  71 ,  (95 , 175 , 95);
        Tea ,  "Tea" ,  72 ,  (95 , 175 , 135);
        Greyblue ,  "Greyblue" ,  73 ,  (95 , 175 , 175);
        SoftBlue2 ,  "SoftBlue2" ,  74 ,  (95 , 175 , 215);
        SkyBlue ,  "SkyBlue" ,  75 ,  (95 , 175 , 255);
        FrogGreen ,  "FrogGreen" ,  76 ,  (95 , 215 , 0);
        LightishGreen ,  "LightishGreen" ,  77 ,  (95 , 215 , 95);
        SoftGreen ,  "SoftGreen" ,  78 ,  (95 , 215 , 135);
        SeafoamBlue ,  "SeafoamBlue" ,  79 ,  (95 , 215 , 175);
        TiffanyBlue ,  "TiffanyBlue" ,  80 ,  (95 , 215 , 215);
        RobinsEgg ,  "RobinsEgg" ,  81 ,  (95 , 215 , 255);
        BrightLimeGreen ,  "BrightLimeGreen" ,  82 ,  (95 , 255 , 0);
        LightBrightGreen ,  "LightBrightGreen" ,  83 ,  (95 , 255 , 95);
        Lightgreen ,  "Lightgreen" ,  84 ,  (95 , 255 , 135);
        LightGreenishBlue ,  "LightGreenishBlue" ,  85 ,  (95 , 255 , 175);
        TiffanyBlue2 ,  "TiffanyBlue2" ,  86 ,  (95 , 255 , 215);
        RobinsEgg2 ,  "RobinsEgg2" ,  87 ,  (95 , 255 , 255);
        DarkRed ,  "DarkRed" ,  88 ,  (135 , 0 , 0);
        DarkMagenta ,  "DarkMagenta" ,  89 ,  (135 , 0 , 95);
        BarneyPurple ,  "BarneyPurple" ,  90 ,  (135 , 0 , 135);
        BarneyPurple2 ,  "BarneyPurple2" ,  91 ,  (135 , 0 , 175);
        Violet ,  "Violet" ,  92 ,  (135 , 0 , 215);
        VividPurple ,  "VividPurple" ,  93 ,  (135 , 0 , 255);
        RusticBronze ,  "RusticBronze" ,  94 ,  (135 , 95 , 0);
        DarkMauve ,  "DarkM4auve" ,  95 ,  (135 , 95 , 95);
        DustyPurple ,  "DustyPurple" ,  96 ,  (135 , 95 , 135);
        DeepLavender ,  "DeepLavender" ,  97 ,  (135 , 95 , 175);
        Purpley ,  "Purpley" ,  98 ,  (135 , 95 , 215);
        Purpley2 ,  "Purpley2" ,  99 ,  (135 , 95 , 255);
        SwampGreen ,  "SwampGreen" ,  100 ,  (135 , 135 , 0);
        BrownGrey ,  "BrownGrey" ,  101 ,  (135 , 135 , 95);
        Grey53 ,  "Grey53" ,  102 ,  (135 , 135 , 135);
        BlueyGrey ,  "BlueyGrey" ,  103 ,  (135 , 135 , 175);
        Perrywinkle ,  "Perrywinkle" ,  104 ,  (135 , 135 , 215);
        LavenderBlue ,  "LavenderBlue" ,  105 ,  (135 , 135 , 255);
        DarkLime ,  "DarkLime" ,  106 ,  (135 , 175 , 0);
        Asparagus ,  "Asparagus" ,  107 ,  (135 , 175 , 95);
        GreyishGreen ,  "GreyishGreen" ,  108 ,  (135 , 175 , 135);
        Bluegrey ,  "Bluegrey" ,  109 ,  (135 , 175 , 175);
        LightGreyBlue ,  "LightGreyBlue" ,  110 ,  (135 , 175 , 215);
        CarolinaBlue ,  "CarolinaBlue" ,  111 ,  (135 , 175 , 255);
        SlimeGreen ,  "SlimeGreen" ,  112 ,  (135 , 215 , 0);
        FreshGreen ,  "FreshGreen" ,  113 ,  (135 , 215 , 95);
        Lichen ,  "Lichen" ,  114 ,  (135 , 215 , 135);
        PaleTeal ,  "PaleTeal" ,  115 ,  (135 , 215 , 175);
        LightTeal ,  "LightTeal" ,  116 ,  (135 , 215 , 215);
        Sky ,  "Sky" ,  117 ,  (135 , 215 , 255);
        BrightLime ,  "BrightLime" ,  118 ,  (135 , 255 , 0);
        LighterGreen ,  "LighterGreen" ,  119 ,  (135 , 255 , 95);
        EasterGreen ,  "EasterGreen" ,  120 ,  (135 , 255 , 135);
        Seafoam ,  "Seafoam" ,  121 ,  (135 , 255 , 175);
        LightAqua ,  "LightAqua" ,  122 ,  (135 , 255 , 215);
        RobinEggBlue ,  "RobinEggBlue" ,  123 ,  (135 , 255 , 255);
        DarkishRed ,  "DarkishRed" ,  124 ,  (175 , 0 , 0);
        VioletRed ,  "VioletRed" ,  125 ,  (175 , 0 , 95);
        BarneyPurple3 ,  "BarneyPurple3" ,  126 ,  (175 , 0 , 135);
        BarneyPurple4 ,  "BarneyPurple4" ,  127 ,  (175 , 0 , 175);
        VibrantPurple ,  "VibrantPurple" ,  128 ,  (175 , 0 , 215);
        BrightViolet ,  "BrightViolet" ,  129 ,  (175 , 0 , 255);
        OrangeyBrown ,  "OrangeyBrown" ,  130 ,  (175 , 95 , 0);
        PinkishBrown ,  "PinkishBrown" ,  131 ,  (175 , 95 , 95);
        Mauve ,  "Mauve" ,  132 ,  (175 , 95 , 135);
        SoftPurple ,  "SoftPurple" ,  133 ,  (175 , 95 , 175);
        LightishPurple ,  "LightishPurple" ,  134 ,  (175 , 95 , 215);
        LighterPurple ,  "LighterPurple" ,  135 ,  (175 , 95 , 255);
        DarkMustard ,  "DarkMustard" ,  136 ,  (175 , 135 , 0);
        DarkSand ,  "DarkSand" ,  137 ,  (175 , 135 , 95);
        Mauve2 ,  "Mauve2" ,  138 ,  (175 , 135 , 135);
        Grey63 ,  "Grey63" ,  139 ,  (175 , 135 , 175);
        PalePurple ,  "PalePurple" ,  140 ,  (175 , 135 , 215);
        Liliac ,  "Liliac" ,  141 ,  (175 , 135 , 255);
        MustardGreen ,  "MustardGreen" ,  142 ,  (175 , 175 , 0);
        Khaki ,  "Khaki" ,  143 ,  (175 , 175 , 95);
        Bland ,  "Bland" ,  144 ,  (175 , 175 , 135);
        Grey69 ,  "Grey69" ,  145 ,  (175 , 175 , 175);
        CloudyBlue ,  "CloudyBlue" ,  146 ,  (175 , 175 , 215);
        PastelBlue ,  "PastelBlue" ,  147 ,  (175 , 175 , 255);
        Bile ,  "Bile" ,  148 ,  (175 , 215 , 0);
        LightOlive ,  "LightOlive" ,  149 ,  (175 , 215 , 95);
        PaleOliveGreen ,  "PaleOliveGreen" ,  150 ,  (175 , 215 , 135);
        LightGreyGreen ,  "LightGreyGreen" ,  151 ,  (175 , 215 , 175);
        LightBlueGrey ,  "LightBlueGrey" ,  152 ,  (175 , 215 , 215);
        PowderBlue ,  "PowderBlue" ,  153 ,  (175 , 215 , 255);
        LemonGreen ,  "LemonGreen" ,  154 ,  (175 , 255 , 0);
        PaleLimeGreen ,  "PaleLimeGreen" ,  155 ,  (175 , 255 , 95);
        Pistachio ,  "Pistachio" ,  156 ,  (175 , 255 , 135);
        LightSeafoamGreen ,  "LightSeafoamGreen" ,  157 ,  (175 , 255 , 175);
        PaleTurquoise ,  "PaleTurquoise" ,  158 ,  (175 , 255 , 215);
        LightCyan ,  "LightCyan" ,  159 ,  (175 , 255 , 255);
        Red2 ,  "Red2" ,  160 ,  (215 , 0 , 0);
        DarkHotPink ,  "DarkHotPink" ,  161 ,  (215 , 0 , 95);
        Magenta ,  "Magenta" ,  162 ,  (215 , 0 , 135);
        BrightPink ,  "BrightPink" ,  163 ,  (215 , 0 , 175);
        Fuchsia2 ,  "Fuchsia2" ,  164 ,  (215 , 0 , 215);
        HotPurple ,  "HotPurple" ,  165 ,  (215 , 0 , 255);
        RustyOrange ,  "RustyOrange" ,  166 ,  (215 , 95 , 0);
        PastelRed ,  "PastelRed" ,  167 ,  (215 , 95 , 95);
        Pinkish ,  "Pinkish" ,  168 ,  (215 , 95 , 135);
        PaleMagenta ,  "PaleMagenta" ,  169 ,  (215 , 95 , 175);
        PinkPurple ,  "PinkPurple" ,  170 ,  (215 , 95 , 215);
        BrightLilac ,  "BrightLilac" ,  171 ,  (215 , 95 , 255);
        Pumpkin ,  "Pumpkin" ,  172 ,  (215 , 135 , 0);
        DarkPeach ,  "DarkPeach" ,  173 ,  (215 , 135 , 95);
        DustyPink ,  "DustyPink" ,  174 ,  (215 , 135 , 135);
        DullPink ,  "DullPink" ,  175 ,  (215 , 135 , 175);
        LavenderPink ,  "LavenderPink" ,  176 ,  (215 , 135 , 215);
        Liliac2 ,  "Liliac2" ,  177 ,  (215 , 135 , 255);
        Mustard ,  "Mustard" ,  178 ,  (215 , 175 , 0);
        Desert ,  "Desert" ,  179 ,  (215 , 175 , 95);
        VeryLightBrown ,  "VeryLightBrown" ,  180 ,  (215 , 175 , 135);
        PinkishGrey ,  "PinkishGrey" ,  181 ,  (215 , 175 , 175);
        Lavender ,  "Lavender" ,  182 ,  (215 , 175 , 215);
        LightViolet ,  "LightViolet" ,  183 ,  (215 , 175 , 255);
        DirtyYellow ,  "DirtyYellow" ,  184 ,  (215 , 215 , 0);
        DullYellow ,  "DullYellow" ,  185 ,  (215 , 215 , 95);
        GreenishBeige ,  "GreenishBeige" ,  186 ,  (215 , 215 , 135);
        Beige ,  "Beige" ,  187 ,  (215 , 215 , 175);
        Grey84 ,  "Grey84" ,  188 ,  (215 , 215 , 215);
        PaleLilac ,  "PaleLilac" ,  189 ,  (215 , 215 , 255);
        NeonYellow ,  "NeonYellow" ,  190 ,  (215 , 255 , 0);
        Pear ,  "Pear" ,  191 ,  (215 , 255 , 95);
        LightYellowGreen ,  "LightYellowGreen" ,  192 ,  (215 , 255 , 135);
        LightLightGreen ,  "LightLightGreen" ,  193 ,  (215 , 255 , 175);
        VeryLightGreen ,  "VeryLightGreen" ,  194 ,  (215 , 255 , 215);
        IceBlue ,  "IceBlue" ,  195 ,  (215 , 255 , 255);
        BrightRed ,  "BrightRed" ,  196 ,  (255 , 0 , 0);
        PinkRed ,  "PinkRed" ,  197 ,  (255 , 0 , 95);
        HotPink ,  "HotPink" ,  198 ,  (255 , 0 , 135);
        BrightPink2 ,  "BrightPink2" ,  199 ,  (255 , 0 , 175);
        HotMagenta ,  "HotMagenta" ,  200 ,  (255 , 0 , 215);
        BrightMagenta ,  "BrightMagenta" ,  201 ,  (255 , 0 , 255);
        BrightOrange ,  "BrightOrange" ,  202 ,  (255 , 95 , 0);
        CoralPink ,  "CoralPink" ,  203 ,  (255 , 95 , 95);
        WarmPink ,  "WarmPink" ,  204 ,  (255 , 95 , 135);
        BubbleGumPink ,  "BubbleGumPink" ,  205 ,  (255 , 95 , 175);
        CandyPink ,  "CandyPink" ,  206 ,  (255 , 95 , 215);
        VioletPink ,  "VioletPink" ,  207 ,  (255 , 95 , 255);
        PumpkinOrange ,  "PumpkinOrange" ,  208 ,  (255 , 135 , 0);
        Melon ,  "Melon" ,  209 ,  (255 , 135 , 95);
        BlushPink ,  "BlushPink" ,  210 ,  (255 , 135 , 135);
        Pinky ,  "Pinky" ,  211 ,  (255 , 135 , 175);
        BubblegumPink ,  "BubblegumPink" ,  212 ,  (255 , 135 , 215);
        PurplyPink ,  "PurplyPink" ,  213 ,  (255 , 135 , 255);
        OrangeYellow ,  "OrangeYellow" ,  214 ,  (255 , 175 , 0);
        PaleOrange ,  "PaleOrange" ,  215 ,  (255 , 175 , 95);
        Peach ,  "Peach" ,  216 ,  (255 , 175 , 135);
        SoftPink ,  "SoftPink" ,  217 ,  (255 , 175 , 175);
        PowderPink ,  "PowderPink" ,  218 ,  (255 , 175 , 215);
        LightLavendar ,  "LightLavendar" ,  219 ,  (255 , 175 , 255);
        SunflowerYellow ,  "SunflowerYellow" ,  220 ,  (255 , 215 , 0);
        LightGold ,  "LightGold" ,  221 ,  (255 , 215 , 95);
        Wheat ,  "Wheat" ,  222 ,  (255 , 215 , 135);
        LightPeach ,  "LightPeach" ,  223 ,  (255 , 215 , 175);
        PalePink ,  "PalePink" ,  224 ,  (255 , 215 , 215);
        PaleMauve ,  "PaleMauve" ,  225 ,  (255 , 215 , 255);
        BrightYellow ,  "BrightYellow" ,  226 ,  (255 , 255 , 0);
        Canary ,  "Canary" ,  227 ,  (255 , 255 , 95);
        PaleYellow ,  "PaleYellow" ,  228 ,  (255 , 255 , 135);
        Parchment ,  "Parchment" ,  229 ,  (255 , 255 , 175);
        Eggshell ,  "Eggshell" ,  230 ,  (255 , 255 , 215);
        Grey100 ,  "Grey100" ,  231 ,  (255 , 255 , 255);
        Grey3 ,  "Grey3" ,  232 ,  (8 , 8 , 8);
        Grey7 ,  "Grey7" ,  233 ,  (18 , 18 , 18);
        Grey11 ,  "Grey11" ,  234 ,  (28 , 28 , 28);
        Grey15 ,  "Grey15" ,  235 ,  (38 , 38 , 38);
        Grey19 ,  "Grey19" ,  236 ,  (48 , 48 , 48);
        Grey23 ,  "Grey23" ,  237 ,  (58 , 58 , 58);
        Grey27 ,  "Grey27" ,  238 ,  (68 , 68 , 68);
        Grey30 ,  "Grey30" ,  239 ,  (78 , 78 , 78);
        Grey35 ,  "Grey35" ,  240 ,  (88 , 88 , 88);
        Grey39 ,  "Grey39" ,  241 ,  (98 , 98 , 98);
        Grey42 ,  "Grey42" ,  242 ,  (108 , 108 , 108);
        Grey46 ,  "Grey46" ,  243 ,  (118 , 118 , 118);
        Grey50 ,  "Grey50" ,  244 ,  (128 , 128 , 128);
        Grey54 ,  "Grey54" ,  245 ,  (138 , 138 , 138);
        Grey58 ,  "Grey58" ,  246 ,  (148 , 148 , 148);
        Grey62 ,  "Grey62" ,  247 ,  (158 , 158 , 158);
        Grey66 ,  "Grey66" ,  248 ,  (168 , 168 , 168);
        Grey70 ,  "Grey70" ,  249 ,  (178 , 178 , 178);
        Grey74 ,  "Grey74" ,  250 ,  (188 , 188 , 188);
        Grey78 ,  "Grey78" ,  251 ,  (198 , 198 , 198);
        Grey82 ,  "Grey82" ,  252 ,  (208 , 208 , 208);
        Grey85 ,  "Grey85" ,  253 ,  (218 , 218 , 218);
        Grey89 ,  "Grey89" ,  254 ,  (228 , 228 , 228);
        Grey93 ,  "Grey93" ,  255 ,  (238 , 238 , 238);

    }
}
