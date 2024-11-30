use super::ColorLibrary;

macro_rules! css_colors {
    (
        $enum_name:ident {
            $($variant_name:ident, $css_name:expr, ($r:expr, $g:expr, $b:expr));* $(;)?
        }
    ) => {
        // Define the enum
        pub enum $enum_name {
            $(
                $variant_name,
            )*
        }

        impl ColorLibrary for $enum_name {
            const WRAPPER: &str = "css()";

            fn wrap_name(str: &str) -> String {
                let wrapped_str = format!("css({})", str);
                return wrapped_str;
            }

            fn unwrap_name(str: &str) -> &str {
                if str.len() < "css()".len() {
                    return str;
                }
                return &str[4..str.len() - 1];
            }

            fn color_name(&self) -> &'static str {
                match self {
                    $(
                        $enum_name::$variant_name => $css_name,
                    )*
                }
            }

            fn get_name(name: &str) -> Option<Self> {
                match name {
                    $(
                        $css_name => Some(Self::$variant_name),
                    )*
                    _ => None
                }
            }

            fn rgb(&self) -> [u8;3] {
                match self {
                    $(
                        $enum_name::$variant_name=> [$r, $g, $b],
                    )*
                }
            }
        }
        // Add utility methods to the enum
    };
}

css_colors! {
    CssColors {
        AliceBlue, "aliceblue", (240, 248, 255);
        AntiqueWhite, "antiquewhite", (250, 235, 215);
        Aqua, "aqua", (0, 255, 255);
        Aquamarine, "aquamarine", (127, 255, 212);
        Azure, "azure", (240, 255, 255);
        Beige, "beige", (245, 245, 220);
        Bisque, "bisque", (255, 228, 196);
        Black, "black", (0, 0, 0);
        BlanchedAlmond, "blanchedalmond", (255, 235, 205);
        Blue, "blue", (0, 0, 255);
        BlueViolet, "blueviolet", (138, 43, 226);
        Brown, "brown", (165, 42, 42);
        BurlyWood, "burlywood", (222, 184, 135);
        CadetBlue, "cadetblue", (95, 158, 160);
        Chartreuse, "chartreuse", (127, 255, 0);
        Chocolate, "chocolate", (210, 105, 30);
        Coral, "coral", (255, 127, 80);
        CornflowerBlue, "cornflowerblue", (100, 149, 237);
        Cornsilk, "cornsilk", (255, 248, 220);
        Crimson, "crimson", (220, 20, 60);
        Cyan, "cyan", (0, 255, 255);
        DarkBlue, "darkblue", (0, 0, 139);
        DarkCyan, "darkcyan", (0, 139, 139);
        DarkGoldenrod, "darkgoldenrod", (184, 134, 11);
        DarkGray, "darkgray", (169, 169, 169);
        DarkGreen, "darkgreen", (0, 100, 0);
        DarkGrey, "darkgrey", (169, 169, 169);
        DarkKhaki, "darkkhaki", (189, 183, 107);
        DarkMagenta, "darkmagenta", (139, 0, 139);
        DarkOliveGreen, "darkolivegreen", (85, 107, 47);
        DarkOrange, "darkorange", (255, 140, 0);
        DarkOrchid, "darkorchid", (153, 50, 204);
        DarkRed, "darkred", (139, 0, 0);
        DarkSalmon, "darksalmon", (233, 150, 122);
        DarkSeaGreen, "darkseagreen", (143, 188, 143);
        DarkSlateBlue, "darkslateblue", (72, 61, 139);
        DarkSlateGray, "darkslategray", (47, 79, 79);
        DarkSlateGrey, "darkslategrey", (47, 79, 79);
        DarkTurquoise, "darkturquoise", (0, 206, 209);
        DarkViolet, "darkviolet", (148, 0, 211);
        DeepPink, "deeppink", (255, 20, 147);
        DeepSkyBlue, "deepskyblue", (0, 191, 255);
        DimGray, "dimgray", (105, 105, 105);
        DimGrey, "dimgrey", (105, 105, 105);
        DodgerBlue, "dodgerblue", (30, 144, 255);
        Firebrick, "firebrick", (178, 34, 34);
        FloralWhite, "floralwhite", (255, 250, 240);
        ForestGreen, "forestgreen", (34, 139, 34);
        Fuchsia, "fuchsia", (255, 0, 255);
        Gainsboro, "gainsboro", (220, 220, 220);
        GhostWhite, "ghostwhite", (248, 248, 255);
        Gold, "gold", (255, 215, 0);
        Goldenrod, "goldenrod", (218, 165, 32);
        Gray, "gray", (128, 128, 128);
        Green, "green", (0, 128, 0);
        GreenYellow, "greenyellow", (173, 255, 47);
        Grey, "grey", (128, 128, 128);
        Honeydew, "honeydew", (240, 255, 240);
        HotPink, "hotpink", (255, 105, 180);
        IndianRed, "indianred", (205, 92, 92);
        Indigo, "indigo", (75, 0, 130);
        Ivory, "ivory", (255, 255, 240);
        Khaki, "khaki", (240, 230, 140);
        Lavender, "lavender", (230, 230, 250);
        LavenderBlush, "lavenderblush", (255, 240, 245);
        LawnGreen, "lawngreen", (124, 252, 0);
        LemonChiffon, "lemonchiffon", (255, 250, 205);
        LightBlue, "lightblue", (173, 216, 230);
        LightCoral, "lightcoral", (240, 128, 128);
        LightCyan, "lightcyan", (224, 255, 255);
        LightGoldenrodYellow, "lightgoldenrodyellow", (250, 250, 210);
        LightGray, "lightgray", (211, 211, 211);
        LightGreen, "lightgreen", (144, 238, 144);
        LightGrey, "lightgrey", (211, 211, 211);
        LightPink, "lightpink", (255, 182, 193);
        LightSalmon, "lightsalmon", (255, 160, 122);
        LightSeaGreen, "lightseagreen", (32, 178, 170);
        LightSkyBlue, "lightskyblue", (135, 206, 250);
        LightSlateGray, "lightslategray", (119, 136, 153);
        LightSlateGrey, "lightslategrey", (119, 136, 153);
        LightSteelBlue, "lightsteelblue", (176, 196, 222);
        LightYellow, "lightyellow", (255, 255, 224);
        Lime, "lime", (0, 255, 0);
        LimeGreen, "limegreen", (50, 205, 50);
        Linen, "linen", (250, 240, 230);
        Magenta, "magenta", (255, 0, 255);
        Maroon, "maroon", (128, 0, 0);
        MediumAquamarine, "mediumaquamarine", (102, 205, 170);
        MediumBlue, "mediumblue", (0, 0, 205);
        MediumOrchid, "mediumorchid", (186, 85, 211);
        MediumPurple, "mediumpurple", (147, 112, 219);
        MediumSeaGreen, "mediumseagreen", (60, 179, 113);
        MediumSlateBlue, "mediumslateblue", (123, 104, 238);
        MediumSpringGreen, "mediumspringgreen", (0, 250, 154);
        MediumTurquoise, "mediumturquoise", (72, 209, 204);
        MediumVioletRed, "mediumvioletred", (199, 21, 133);
        MidnightBlue, "midnightblue", (25, 25, 112);
        MintCream, "mintcream", (245, 255, 250);
        MistyRose, "mistyrose", (255, 228, 225);
        Moccasin, "moccasin", (255, 228, 181);
        NavajoWhite, "navajowhite", (255, 222, 173);
        Navy, "navy", (0, 0, 128);
        OldLace, "oldlace", (253, 245, 230);
        Olive, "olive", (128, 128, 0);
        OliveDrab, "olivedrab", (107, 142, 35);
        Orange, "orange", (255, 165, 0);
        OrangeRed, "orangered", (255, 69, 0);
        Orchid, "orchid", (218, 112, 214);
        PaleGoldenrod, "palegoldenrod", (238, 232, 170);
        PaleGreen, "palegreen", (152, 251, 152);
        PaleTurquoise, "paleturquoise", (175, 238, 238);
        PaleVioletRed, "palevioletred", (219, 112, 147);
        PapayaWhip, "papayawhip", (255, 239, 213);
        PeachPuff, "peachpuff", (255, 218, 185);
        Peru, "peru", (205, 133, 63);
        Pink, "pink", (255, 192, 203);
        Plum, "plum", (221, 160, 221);
        PowderBlue, "powderblue", (176, 224, 230);
        Purple, "purple", (128, 0, 128);
        RebeccaPurple, "rebeccapurple", (102, 51, 153);
        Red, "red", (255, 0, 0);
        RosyBrown, "rosybrown", (188, 143, 143);
        RoyalBlue, "royalblue", (65, 105, 225);
        SaddleBrown, "saddlebrown", (139, 69, 19);
        Salmon, "salmon", (250, 128, 114);
        SandyBrown, "sandybrown", (244, 164, 96);
        SeaGreen, "seagreen", (46, 139, 87);
        SeaShell, "seashell", (255, 245, 238);
        Sienna, "sienna", (160, 82, 45);
        Silver, "silver", (192, 192, 192);
        SkyBlue, "skyblue", (135, 206, 235);
        SlateBlue, "slateblue", (106, 90, 205);
        SlateGray, "slategray", (112, 128, 144);
        SlateGrey, "slategrey", (112, 128, 144);
        Snow, "snow", (255, 250, 250);
        SpringGreen, "springgreen", (0, 255, 127);
        SteelBlue, "steelblue", (70, 130, 180);
        Tan, "tan", (210, 180, 140);
        Teal, "teal", (0, 128, 128);
        Thistle, "thistle", (216, 191, 216);
        Tomato, "tomato", (255, 99, 71);
        Turquoise, "turquoise", (64, 224, 208);
        Violet, "violet", (238, 130, 238);
        Wheat, "wheat", (245, 222, 179);
        White, "white", (255, 255, 255);
        WhiteSmoke, "whitesmoke", (245, 245, 245);
        Yellow, "yellow", (255, 255, 0);
        YellowGreen, "yellowgreen", (154, 205, 50);
    }
}
