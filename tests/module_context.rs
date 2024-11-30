use nenyr::NenyrParser;

#[test]
fn module_context_is_valid() {
    let mut parser = NenyrParser::new();

    match std::fs::read_to_string("mocks/nenyr/module.nyr") {
        Ok(raw_nenyr) => {
            let central_ast =
                parser.parse(raw_nenyr.to_string(), "mocks/nenyr/module.nyr".to_string());

            assert_eq!(
                format!("{:?}", central_ast),
                "Ok(ModuleContext(ModuleContext { module_name: \"modernCanvas\", extending_from: Some(\"dynamicLayout\"), aliases: Some(NenyrAliases { values: {\"bgd\": \"background\", \"bgdColor\": \"background-color\", \"bgdImg\": \"background-image\", \"bgdSize\": \"background-size\", \"bd\": \"border\", \"bdT\": \"border-top\", \"bdB\": \"border-bottom\", \"bdL\": \"border-left\", \"bdR\": \"border-right\", \"bdColor\": \"border-color\", \"bdRadius\": \"border-radius\", \"boxShdw\": \"box-shadow\", \"dp\": \"display\", \"pos\": \"position\", \"flt\": \"float\", \"ovf\": \"overflow\", \"ovfX\": \"overflow-x\", \"ovfY\": \"overflow-y\", \"zIdx\": \"z-index\", \"flexDir\": \"flex-direction\", \"flexWrp\": \"flex-wrap\", \"algnItems\": \"align-items\", \"justifyCnt\": \"justify-content\", \"gridTpl\": \"grid-template\", \"wd\": \"width\", \"hgt\": \"height\", \"maxWd\": \"max-width\", \"minWd\": \"min-width\", \"maxHgt\": \"max-height\", \"minHgt\": \"min-height\", \"mg\": \"margin\", \"mgT\": \"margin-top\", \"mgB\": \"margin-bottom\", \"mgL\": \"margin-left\", \"mgR\": \"margin-right\", \"pdg\": \"padding\", \"pdgT\": \"padding-top\", \"pdgB\": \"padding-bottom\", \"pdgL\": \"padding-left\", \"pdgR\": \"padding-right\", \"gp\": \"gap\", \"fntSize\": \"font-size\", \"fntWeight\": \"font-weight\", \"fntFam\": \"font-family\", \"txtAlign\": \"text-align\", \"txtDec\": \"text-decoration\", \"txtTrnsf\": \"text-transform\", \"lineHgt\": \"line-height\", \"letterSpc\": \"letter-spacing\", \"wordSpc\": \"word-spacing\", \"clr\": \"color\", \"opcty\": \"opacity\", \"trnsfrm\": \"transform\", \"trnsfrmOrgn\": \"transform-origin\", \"trnstn\": \"transition\", \"trnstnDur\": \"transition-duration\", \"crsr\": \"cursor\", \"vis\": \"visibility\", \"fltShdw\": \"filter\"} }), variables: Some(NenyrVariables { values: {\"myColor\": \"#FF6677\", \"grayColor\": \"gray\", \"blueColor\": \"blue\", \"redColor\": \"red\", \"primaryColor\": \"yellow\", \"secondaryColor\": \"white\", \"accColor\": \"#FF5733\", \"darkGrayColor\": \"#333333\", \"lightGrayColor\": \"#D3D3D3\", \"bgdColor\": \"#FAFAFA\", \"borColor\": \"#CCCCCC\", \"highlightColor\": \"#FFD700\", \"shadowColor\": \"rgba(0, 0, 0, 0.2)\", \"linkColor\": \"#1E90FF\", \"successColor\": \"#4CAF50\", \"warningColor\": \"#FFA500\", \"dangerColor\": \"#DC143C\"} }), animations: Some({\"slideScale\": NenyrAnimation { animation_name: \"slideScale\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [20.0], properties: {\"transform\": \"translateX(10%) scale(1.1)\"} }, Fraction { stops: [40.0, 60.0], properties: {\"transform\": \"translateX(30%) scale(1.2)\"} }, Fraction { stops: [80.0], properties: {\"transform\": \"translateX(50%) scale(0.9)\"} }, Fraction { stops: [100.0], properties: {\"transform\": \"translateX(0) scale(1)\"} }] }, \"fadeColorChange\": NenyrAnimation { animation_name: \"fadeColorChange\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [10.0], properties: {\"opacity\": \"0.1\", \"background-color\": \"${primaryColorVar}\"} }, Fraction { stops: [30.0, 60.0], properties: {\"opacity\": \"0.5\", \"background-color\": \"green\"} }, Fraction { stops: [90.0], properties: {\"opacity\": \"1\", \"background-color\": \"${secondaryColorVar}\"} }, Fraction { stops: [100.0], properties: {\"opacity\": \"0.8\", \"background-color\": \"purple\"} }] }, \"rotateScale\": NenyrAnimation { animation_name: \"rotateScale\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [25.0], properties: {\"transform\": \"rotate(15deg) scale(1.05)\"} }, Fraction { stops: [50.0, 75.0], properties: {\"transform\": \"rotate(30deg) scale(0.95)\"} }, Fraction { stops: [90.0], properties: {\"transform\": \"rotate(45deg) scale(1.15)\"} }, Fraction { stops: [100.0], properties: {\"transform\": \"rotate(0deg) scale(1)\"} }] }, \"borderFlash\": NenyrAnimation { animation_name: \"borderFlash\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [10.0], properties: {\"border-color\": \"${accentColorVar}\", \"border-width\": \"1px\"} }, Fraction { stops: [30.0, 50.0, 70.0], properties: {\"border-color\": \"red\", \"border-width\": \"3px\"} }, Fraction { stops: [90.0], properties: {\"border-color\": \"green\", \"border-width\": \"2px\"} }, Fraction { stops: [100.0], properties: {\"border-color\": \"${accentColorVar}\", \"border-width\": \"1px\"} }] }, \"bounceOpacity\": NenyrAnimation { animation_name: \"bounceOpacity\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [15.0], properties: {\"transform\": \"translateY(-20%)\", \"opacity\": \"0.3\"} }, Fraction { stops: [45.0, 65.0], properties: {\"transform\": \"translateY(0)\", \"opacity\": \"1\"} }, Fraction { stops: [85.0], properties: {\"transform\": \"translateY(20%)\", \"opacity\": \"0.7\"} }, Fraction { stops: [100.0], properties: {\"transform\": \"translateY(0)\", \"opacity\": \"1\"} }] }, \"floatScaleOpacity\": NenyrAnimation { animation_name: \"floatScaleOpacity\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [10.5], properties: {\"transform\": \"scale(0.8)\", \"opacity\": \"0.5\"} }, Fraction { stops: [25.5, 50.75], properties: {\"transform\": \"scale(1.2)\", \"opacity\": \"0.8\"} }, Fraction { stops: [75.25], properties: {\"transform\": \"scale(1.05)\", \"opacity\": \"1\"} }, Fraction { stops: [100.0], properties: {\"transform\": \"scale(1)\", \"opacity\": \"0.9\"} }] }, \"smoothColorFade\": NenyrAnimation { animation_name: \"smoothColorFade\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [5.5], properties: {\"background-color\": \"${highlightColorVar}\", \"opacity\": \"0.2\"} }, Fraction { stops: [30.25, 60.5], properties: {\"background-color\": \"lightblue\", \"opacity\": \"0.6\"} }, Fraction { stops: [85.75], properties: {\"background-color\": \"lightcoral\", \"opacity\": \"0.9\"} }, Fraction { stops: [100.0], properties: {\"background-color\": \"${backgroundColorVar}\", \"opacity\": \"1\"} }] }, \"complexRotateScale\": NenyrAnimation { animation_name: \"complexRotateScale\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [15.5], properties: {\"transform\": \"rotate(12.5deg) scale(0.95)\"} }, Fraction { stops: [40.25, 65.75], properties: {\"transform\": \"rotate(25.5deg) scale(1.1)\"} }, Fraction { stops: [85.5], properties: {\"transform\": \"rotate(37.5deg) scale(0.8)\"} }, Fraction { stops: [100.0], properties: {\"transform\": \"rotate(0deg) scale(1)\"} }] }, \"floatMoveOpacity\": NenyrAnimation { animation_name: \"floatMoveOpacity\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [8.5], properties: {\"transform\": \"translateY(-10.5%)\", \"opacity\": \"0.3\"} }, Fraction { stops: [35.5, 55.25], properties: {\"transform\": \"translateY(0)\", \"opacity\": \"1\"} }, Fraction { stops: [78.75], properties: {\"transform\": \"translateY(15.75%)\", \"opacity\": \"0.7\"} }, Fraction { stops: [100.0], properties: {\"transform\": \"translateY(0)\", \"opacity\": \"1\"} }] }, \"floatBorderFlash\": NenyrAnimation { animation_name: \"floatBorderFlash\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [12.5], properties: {\"border-color\": \"${accentColorVar}\", \"border-width\": \"1px\"} }, Fraction { stops: [35.75, 58.5, 78.25], properties: {\"border-color\": \"orange\", \"border-width\": \"3px\"} }, Fraction { stops: [90.5], properties: {\"border-color\": \"teal\", \"border-width\": \"2px\"} }, Fraction { stops: [100.0], properties: {\"border-color\": \"${accentColorVar}\", \"border-width\": \"1px\"} }] }, \"horizontalMove\": NenyrAnimation { animation_name: \"horizontalMove\", kind: Some(Progressive), progressive_count: Some(5), keyframe: [Progressive({\"transform\": \"translateX(0)\", \"background-color\": \"lightgray\"}), Progressive({\"transform\": \"translateX(50px)\", \"background-color\": \"lightblue\"}), Progressive({\"transform\": \"translateX(100px)\", \"background-color\": \"lightgreen\"}), Progressive({\"transform\": \"translateX(150px)\", \"background-color\": \"lightcoral\"}), Progressive({\"transform\": \"translateX(200px)\", \"background-color\": \"lightgoldenrodyellow\"})] }, \"fadeScale\": NenyrAnimation { animation_name: \"fadeScale\", kind: Some(Progressive), progressive_count: Some(4), keyframe: [Progressive({\"opacity\": \"0.2\", \"transform\": \"scale(0.8)\"}), Progressive({\"opacity\": \"0.5\", \"transform\": \"scale(1)\"}), Progressive({\"opacity\": \"0.8\", \"transform\": \"scale(1.2)\"}), Progressive({\"opacity\": \"1\", \"transform\": \"scale(1.1)\"})] }, \"colorBorderSize\": NenyrAnimation { animation_name: \"colorBorderSize\", kind: Some(Progressive), progressive_count: Some(5), keyframe: [Progressive({\"background-color\": \"lavender\", \"border\": \"2px solid ${primaryColorVar}\", \"height\": \"50px\", \"width\": \"50px\"}), Progressive({\"background-color\": \"lightpink\", \"border\": \"4px solid ${secondaryColorVar}\", \"height\": \"75px\", \"width\": \"75px\"}), Progressive({\"background-color\": \"lightyellow\", \"border\": \"6px solid ${accentColorVar}\", \"height\": \"100px\", \"width\": \"100px\"}), Progressive({\"background-color\": \"lightgreen\", \"border\": \"8px solid teal\", \"height\": \"125px\", \"width\": \"125px\"}), Progressive({\"background-color\": \"lightblue\", \"border\": \"10px solid navy\", \"height\": \"150px\", \"width\": \"150px\"})] }, \"rotateColorChange\": NenyrAnimation { animation_name: \"rotateColorChange\", kind: Some(Progressive), progressive_count: Some(5), keyframe: [Progressive({\"transform\": \"rotate(0deg)\", \"background-color\": \"white\"}), Progressive({\"transform\": \"rotate(45deg)\", \"background-color\": \"lightgray\"}), Progressive({\"transform\": \"rotate(90deg)\", \"background-color\": \"lightblue\"}), Progressive({\"transform\": \"rotate(135deg)\", \"background-color\": \"lightgreen\"}), Progressive({\"transform\": \"rotate(180deg)\", \"background-color\": \"lavender\"})] }, \"verticalBounce\": NenyrAnimation { animation_name: \"verticalBounce\", kind: Some(Progressive), progressive_count: Some(5), keyframe: [Progressive({\"transform\": \"translateY(0)\", \"border\": \"2px dashed ${highlightColorVar}\"}), Progressive({\"transform\": \"translateY(-20px)\", \"border\": \"2px solid orange\"}), Progressive({\"transform\": \"translateY(0)\", \"border\": \"3px solid ${highlightColorVar}\"}), Progressive({\"transform\": \"translateY(20px)\", \"border\": \"4px dotted teal\"}), Progressive({\"transform\": \"translateY(0)\", \"border\": \"2px dashed ${highlightColorVar}\"})] }, \"fadeAndScale\": NenyrAnimation { animation_name: \"fadeAndScale\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"opacity\": \"0\", \"transform\": \"scale(0.5)\"}), Halfway({\"opacity\": \"0.5\", \"transform\": \"scale(1)\"}), To({\"opacity\": \"1\", \"transform\": \"scale(1.2)\"})] }, \"colorAndBorderChange\": NenyrAnimation { animation_name: \"colorAndBorderChange\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"background-color\": \"lightgray\", \"border\": \"2px solid ${accentColorVar}\"}), Halfway({\"background-color\": \"lightblue\", \"border\": \"4px solid ${highlightColorVar}\"}), To({\"background-color\": \"lightgreen\", \"border\": \"6px solid teal\"})] }, \"verticalMoveAndRotate\": NenyrAnimation { animation_name: \"verticalMoveAndRotate\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"transform\": \"translateY(0) rotate(0deg)\"}), Halfway({\"transform\": \"translateY(-20px) rotate(45deg)\"}), To({\"transform\": \"translateY(0) rotate(90deg)\"})] }, \"textFadeAndColorChange\": NenyrAnimation { animation_name: \"textFadeAndColorChange\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"color\": \"${primaryTextColorVar}\", \"opacity\": \"0.2\"}), Halfway({\"color\": \"${secondaryTextColorVar}\", \"opacity\": \"0.6\"}), To({\"color\": \"darkblue\", \"opacity\": \"1\"})] }, \"expandWidthHeight\": NenyrAnimation { animation_name: \"expandWidthHeight\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"width\": \"50px\", \"height\": \"50px\"}), Halfway({\"width\": \"100px\", \"height\": \"100px\"}), To({\"width\": \"150px\", \"height\": \"150px\"})] }, \"borderColorChange\": NenyrAnimation { animation_name: \"borderColorChange\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"border\": \"2px dashed ${myColorVar}\", \"background-color\": \"lightyellow\"}), Halfway({\"border\": \"4px dotted ${secondaryColorVar}\", \"background-color\": \"lightpink\"}), To({\"border\": \"6px solid ${highlightColorVar}\", \"background-color\": \"lavender\"})] }, \"translateAndScale\": NenyrAnimation { animation_name: \"translateAndScale\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"transform\": \"translateX(0) scale(1)\"}), Halfway({\"transform\": \"translateX(50px) scale(1.5)\"}), To({\"transform\": \"translateX(100px) scale(1)\"})] }}), classes: Some({\"celestialHeron\": NenyrStyleClass { class_name: \"celestialHeron\", deriving_from: Some(\"stardustFeather\"), is_important: Some(true), style_patterns: Some({\"_stylesheet\": {\"nickname;bgdColor\": \"${primaryColor}\", \"nickname;clr\": \"${accColor}\", \"nickname;pdg\": \"${m20px30}\", \"nickname;dp\": \"flex\", \"align-items\": \"center\"}, \":hover\": {\"nickname;clr\": \"${secondaryColor}\", \"nickname;bd\": \"2px solid ${primaryColor}\"}, \"::after\": {\"content\": \"' '\", \"nickname;dp\": \"block\", \"nickname;wd\": \"100%\", \"nickname;hgt\": \"2px\", \"nickname;bgd\": \"${secondaryColor}\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"nickname;dp\": \"block\", \"nickname;flexDir\": \"column\", \"nickname;pdg\": \"${m8px12}\"}}, \"onDeskDesktop\": {\":hover\": {\"nickname;bgd\": \"${secondaryColor}\", \"nickname;pdg\": \"${m15px}\"}}}) }, \"ancientPhoenix\": NenyrStyleClass { class_name: \"ancientPhoenix\", deriving_from: Some(\"fieryAura\"), is_important: None, style_patterns: Some({\"_stylesheet\": {\"nickname;bgdColor\": \"${accColor}\", \"nickname;clr\": \"${primaryColor}\", \"nickname;fntSize\": \"1.2em\", \"nickname;pdg\": \"${m12px18}\", \"nickname;txtAlign\": \"center\", \"nickname;bdRadius\": \"8px\"}, \":hover\": {\"nickname;bgd\": \"${primaryColor}\", \"nickname;clr\": \"${secondaryColor}\", \"nickname;boxShdw\": \"0 4px 8px ${shadowColor}\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"nickname;wd\": \"100%\", \"nickname;pdg\": \"${m8px12}\", \"nickname;fntSize\": \"1em\"}}, \"onDeskDesktop\": {\"::after\": {\"content\": \"'🔥'\", \"nickname;pos\": \"absolute\", \"right\": \"5px\", \"top\": \"5px\"}}}) }, \"emeraldRaven\": NenyrStyleClass { class_name: \"emeraldRaven\", deriving_from: Some(\"mysticShroud\"), is_important: Some(true), style_patterns: Some({\"_stylesheet\": {\"nickname;bgd\": \"${secondaryColor}\", \"nickname;bd\": \"3px solid ${primaryColor}\", \"nickname;bdRadius\": \"10px\", \"nickname;pdg\": \"${m20px30}\", \"text-shadow\": \"1px 1px 2px ${accColor}\"}, \":hover\": {\"nickname;bgdColor\": \"${primaryColor}\", \"nickname;clr\": \"${accColor}\", \"nickname;boxShdw\": \"0 6px 12px ${shadowColor}\"}, \"::before\": {\"content\": \"' '\", \"nickname;dp\": \"block\", \"nickname;wd\": \"100%\", \"nickname;hgt\": \"4px\", \"nickname;bgd\": \"${accColor}\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"nickname;pdg\": \"${m15px20}\", \"nickname;fntSize\": \"0.9em\", \"nickname;bdRadius\": \"5px\"}}, \"onDeskDesktop\": {\"_stylesheet\": {\"nickname;pdg\": \"${m15px20}\", \"nickname;fntSize\": \"0.9em\", \"nickname;bdRadius\": \"5px\"}, \":hover\": {\"nickname;clr\": \"${secondaryColor}\", \"nickname;bgd\": \"${accColor}\"}, \"::after\": {\"content\": \"' '\", \"nickname;dp\": \"block\", \"nickname;wd\": \"50%\", \"nickname;hgt\": \"2px\", \"nickname;bgd\": \"${primaryColor}\", \"nickname;mgT\": \"10px\", \"nickname;mgB\": \"0\"}}}) }, \"nebulousLion\": NenyrStyleClass { class_name: \"nebulousLion\", deriving_from: Some(\"stellarMane\"), is_important: None, style_patterns: Some({\"_stylesheet\": {\"nickname;bgd\": \"${secondaryColor}\", \"nickname;pdg\": \"${m12px20}\", \"nickname;clr\": \"${primaryColor}\", \"nickname;fntWeight\": \"bold\", \"nickname;letterSpc\": \"0.1em\", \"nickname;bd\": \"1px solid ${accColor}\"}, \":hover\": {\"nickname;bgd\": \"${accColor}\", \"nickname;clr\": \"${primaryColor}\", \"nickname;boxShdw\": \"0 6px 12px ${shadowColor}\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"nickname;pdg\": \"${m10px16}\", \"nickname;fntSize\": \"1em\"}}, \"onDeskDesktop\": {\"_stylesheet\": {\"nickname;pdg\": \"${m15px25}\", \"nickname;fntSize\": \"1.1em\"}, \"::after\": {\"content\": \"'✨'\", \"nickname;pos\": \"absolute\", \"top\": \"10px\", \"left\": \"10px\", \"nickname;fntSize\": \"1.5em\"}, \":hover\": {\"nickname;bgd\": \"${accColor}\", \"nickname;clr\": \"${primaryColor}\", \"nickname;boxShdw\": \"0 6px 12px ${shadowColor}\"}}}) }, \"luminousDragon\": NenyrStyleClass { class_name: \"luminousDragon\", deriving_from: Some(\"radiantWings\"), is_important: Some(true), style_patterns: Some({\"_stylesheet\": {\"background-color\": \"${primaryColor}\", \"color\": \"${accColor}\", \"padding\": \"${m20px30}\", \"display\": \"flex\", \"align-items\": \"center\"}, \":hover\": {\"color\": \"${secondaryColor}\", \"border\": \"2px solid ${primaryColor}\"}, \"::after\": {\"content\": \"''\", \"display\": \"block\", \"width\": \"100%\", \"height\": \"2px\", \"background\": \"${secondaryColor}\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"display\": \"block\", \"flex-direction\": \"column\", \"padding\": \"${m8px12}\"}}, \"onDeskDesktop\": {\":hover\": {\"background\": \"${secondaryColor}\", \"padding\": \"${m15px}\"}}}) }, \"ancientGuardian\": NenyrStyleClass { class_name: \"ancientGuardian\", deriving_from: Some(\"fieryEmber\"), is_important: None, style_patterns: Some({\"_stylesheet\": {\"background-color\": \"${accColor}\", \"color\": \"${primaryColor}\", \"font-size\": \"1.2em\", \"padding\": \"${m12px18}\", \"text-align\": \"center\", \"border-radius\": \"8px\"}, \":hover\": {\"background\": \"${primaryColor}\", \"color\": \"${secondaryColor}\", \"box-shadow\": \"0 4px 8px ${shadowColor}\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"width\": \"100%\", \"padding\": \"${m8px12}\", \"font-size\": \"1em\"}}, \"onDeskDesktop\": {\"::after\": {\"content\": \"'🔥'\", \"position\": \"absolute\", \"right\": \"5px\", \"top\": \"5px\"}}}) }, \"mysticalPhoenix\": NenyrStyleClass { class_name: \"mysticalPhoenix\", deriving_from: Some(\"fieryWings\"), is_important: Some(true), style_patterns: Some({\"_stylesheet\": {\"background\": \"${secondaryColor}\", \"border\": \"3px solid ${primaryColor}\", \"border-radius\": \"10px\", \"padding\": \"${m20px30}\", \"text-shadow\": \"1px 1px 2px ${accColor}\"}, \":hover\": {\"background-color\": \"${primaryColor}\", \"color\": \"${accColor}\", \"box-shadow\": \"0 6px 12px ${shadowColor}\"}, \"::before\": {\"content\": \"''\", \"display\": \"block\", \"width\": \"100%\", \"height\": \"4px\", \"background\": \"${accColor}\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"padding\": \"${m15px20}\", \"font-size\": \"0.9em\", \"border-radius\": \"5px\"}, \":hover\": {\"color\": \"${secondaryColor}\", \"background\": \"${accColor}\"}, \"::after\": {\"content\": \"''\", \"display\": \"block\", \"width\": \"50%\", \"height\": \"2px\", \"background\": \"${primaryColor}\", \"margin-top\": \"10px\", \"margin-bottom\": \"0\"}}, \"onDeskDesktop\": {\":hover\": {\"color\": \"${secondaryColor}\", \"background\": \"${accColor}\"}, \"::after\": {\"content\": \"''\", \"display\": \"block\", \"width\": \"50%\", \"height\": \"2px\", \"background\": \"${primaryColor}\", \"margin-top\": \"10px\", \"margin-bottom\": \"0\"}}}) }, \"celestialLion\": NenyrStyleClass { class_name: \"celestialLion\", deriving_from: Some(\"stellarPride\"), is_important: None, style_patterns: Some({\"_stylesheet\": {\"background\": \"${secondaryColor}\", \"padding\": \"${m12px20}\", \"color\": \"${primaryColor}\", \"font-weight\": \"bold\", \"letter-spacing\": \"0.1em\", \"border\": \"1px solid ${accColor}\"}, \":hover\": {\"background\": \"${accColor}\", \"color\": \"${primaryColor}\", \"box-shadow\": \"0 6px 12px ${shadowColor}\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"padding\": \"${m10px16}\", \"font-size\": \"1em\"}, \":hover\": {\"background\": \"${accColor}\", \"color\": \"${primaryColor}\", \"box-shadow\": \"0 6px 12px ${shadowColor}\"}, \"::after\": {\"content\": \"'✨'\", \"position\": \"absolute\", \"top\": \"10px\", \"left\": \"10px\", \"font-size\": \"1.5em\"}, \"::before\": {\"position\": \"absolute\", \"top\": \"10px\", \"left\": \"10px\"}}, \"onDeskDesktop\": {\"_stylesheet\": {\"padding\": \"${m15px25}\", \"font-size\": \"1.1em\"}, \":hover\": {\"background\": \"${accColor}\", \"color\": \"${primaryColor}\", \"box-shadow\": \"0 6px 12px ${shadowColor}\"}, \"::after\": {\"content\": \"'✨'\", \"position\": \"absolute\", \"top\": \"10px\", \"left\": \"10px\", \"font-size\": \"1.5em\"}, \"::before\": {\"position\": \"absolute\", \"top\": \"10px\", \"left\": \"10px\"}}}) }}) }))".to_string()
            );
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
