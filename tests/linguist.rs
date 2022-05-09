//
// Copyright (c) 2018-2020 Bahtiar `kalkin-` Gadimov.
//
// This file is part of file-expert
// (see https://github.com/kalkin/file-expert).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
#![cfg(test)]
#![cfg(not(tarpaulin_include))]
#![allow(non_snake_case)]

mod _1c_enterprise {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path =
            Path::new(&"./samples/1C Enterprise/CommonModule.ОбменМобильныеОбщее.Module.bsl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("1C Enterprise".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/1C Enterprise/ci_before_script.os");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("1C Enterprise".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(
            &"./samples/1C Enterprise/Catalog.ИсходящиеПисьма.Form.ФормаЭлемента.Form.Module.bsl",
        );
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("1C Enterprise".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/1C Enterprise/Document.РасходТовара.ObjectModule.bsl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("1C Enterprise".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(
            &"./samples/1C Enterprise/Catalog.Товары.Command.ПечатьПрайсЛиста.CommandModule.bsl",
        );
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("1C Enterprise".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/1C Enterprise/test_canCompile.os");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("1C Enterprise".to_string());
        assert_eq!(actual, expected);
    }
}

mod _4d {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/4D/play_with_classes.4dm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("4D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/4D/webArea.4dm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("4D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/4D/generate_project.4dm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("4D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/4D/test_webServerStart.4dm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("4D".to_string());
        assert_eq!(actual, expected);
    }
}

mod abap {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ABAP/cl_csv_parser.abap");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ABAP".to_string());
        assert_eq!(actual, expected);
    }
}

mod abap_cds {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ABAP CDS/zcds_monsters_parameters.ddls.asddls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ABAP CDS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/ABAP CDS/zcds_monsters_association.ddls.asddls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ABAP CDS".to_string());
        assert_eq!(actual, expected);
    }
}

mod abnf {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ABNF/toml.abnf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ABNF".to_string());
        assert_eq!(actual, expected);
    }
}

mod ags_script {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/AGS Script/KeyboardMovement_102.asc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AGS Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/AGS Script/KeyboardMovement_102.ash");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AGS Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/AGS Script/GlobalScript.ash");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AGS Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/AGS Script/GlobalScript.asc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AGS Script".to_string());
        assert_eq!(actual, expected);
    }
}

mod aidl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/AIDL/IVoid.aidl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AIDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/AIDL/ExtendableParcelable.aidl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AIDL".to_string());
        assert_eq!(actual, expected);
    }
}

mod al {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/AL/ALIssueList.al");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/AL/RefreshALIssuesCode.al");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/AL/ALIssue.al");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AL".to_string());
        assert_eq!(actual, expected);
    }
}

mod ampl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/AMPL/toy.ampl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AMPL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/AMPL/CT2.mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AMPL".to_string());
        assert_eq!(actual, expected);
    }
}

mod api_blueprint {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/API Blueprint/actions.apib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("API Blueprint".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/API Blueprint/attributes.apib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("API Blueprint".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/API Blueprint/simple.apib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("API Blueprint".to_string());
        assert_eq!(actual, expected);
    }
}

mod apl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/APL/hashbang");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("APL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/APL/UT.dyalog");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("APL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/APL/DeepakChopra.apl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("APL".to_string());
        assert_eq!(actual, expected);
    }
}

mod asl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ASL/example.dsl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ASL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/ASL/example.asl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ASL".to_string());
        assert_eq!(actual, expected);
    }
}

mod asn_dot_1 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ASN.1/example.asn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ASN.1".to_string());
        assert_eq!(actual, expected);
    }
}

mod asp_dot_net {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ASP.NET/Login.aspx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ASP.NET".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/ASP.NET/OpenAuthProviders.ascx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ASP.NET".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/ASP.NET/Global.asax");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ASP.NET".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/ASP.NET/EchoSocket.ashx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ASP.NET".to_string());
        assert_eq!(actual, expected);
    }
}

mod ats {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ATS/intinf_vt.dats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ATS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/ATS/csv_parse.hats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ATS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/ATS/DiningPhil2.dats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ATS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/ATS/basis_ssntype.sats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ATS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/ATS/DiningPhil2_thread.dats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ATS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/ATS/DiningPhil2_fork.dats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ATS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/ATS/YonedaLemma.dats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ATS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/ATS/DiningPhil2.sats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ATS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/ATS/CoYonedaLemma.dats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ATS".to_string());
        assert_eq!(actual, expected);
    }
}

mod actionscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ActionScript/TextFieldUtil.as");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ActionScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/ActionScript/FooBar.as");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ActionScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/ActionScript/NumberUtil.as");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ActionScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/ActionScript/HelloWorld.as");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ActionScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod adobe_font_metrics {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Adobe Font Metrics/lambda.afm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Adobe Font Metrics".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Adobe Font Metrics/OpenSansCondensed-Bold.afm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Adobe Font Metrics".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Adobe Font Metrics/SpecialElite.afm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Adobe Font Metrics".to_string());
        assert_eq!(actual, expected);
    }
}

mod agda {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Agda/NatCat.agda");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Agda".to_string());
        assert_eq!(actual, expected);
    }
}

mod alloy {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Alloy/marksweepgc.als");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Alloy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Alloy/views.als");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Alloy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Alloy/file_system.als");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Alloy".to_string());
        assert_eq!(actual, expected);
    }
}

mod alpine_abuild {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Alpine Abuild/filenames/APKBUILD");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Alpine Abuild".to_string());
        assert_eq!(actual, expected);
    }
}

mod altium_designer {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Altium Designer/Sample Schematic Sheet.SchDoc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Altium Designer".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Altium Designer/Sample Board Design.PcbDoc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Altium Designer".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Altium Designer/Sample Altium Project.PrjPcb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Altium Designer".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Altium Designer/Sample Output Job.OutJob");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Altium Designer".to_string());
        assert_eq!(actual, expected);
    }
}

mod angelscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/AngelScript/botmanager.as");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AngelScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/AngelScript/payload.as");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AngelScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod ant_build_system {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Ant Build System/filenames/ant.xml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ant Build System".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Ant Build System/filenames/build.xml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ant Build System".to_string());
        assert_eq!(actual, expected);
    }
}

mod apacheconf {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ApacheConf/apache.vhost");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ApacheConf".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/ApacheConf/filenames/httpd.conf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ApacheConf".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/ApacheConf/filenames/.htaccess");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ApacheConf".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/ApacheConf/filenames/apache2.conf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ApacheConf".to_string());
        assert_eq!(actual, expected);
    }
}

mod apex {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Apex/ArrayUtils.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Apex".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Apex/GeoUtils.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Apex".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Apex/LanguageUtils.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Apex".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Apex/EmailUtils.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Apex".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Apex/BooleanUtils.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Apex".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Apex/TwilioAPI.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Apex".to_string());
        assert_eq!(actual, expected);
    }
}

mod apollo_guidance_computer {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(
            &"./samples/Apollo Guidance Computer/BURN_BABY_BURN--MASTER_IGNITION_ROUTINE.agc",
        );
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Apollo Guidance Computer".to_string());
        assert_eq!(actual, expected);
    }
}

mod applescript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/AppleScript/center.applescript");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AppleScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/AppleScript/Time Of Day.applescript");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AppleScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/AppleScript/Crazy Message Text.applescript");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AppleScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/AppleScript/Count Messages in All Mailboxes.applescript");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AppleScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/AppleScript/Get User Name.applescript");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AppleScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/AppleScript/Convert To PostScript.applescript");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AppleScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/AppleScript/Convert To PDF.applescript");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AppleScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod asciidoc {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/AsciiDoc/sample.adoc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AsciiDoc".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/AsciiDoc/encoding.asciidoc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AsciiDoc".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/AsciiDoc/list.asc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AsciiDoc".to_string());
        assert_eq!(actual, expected);
    }
}

mod aspectj {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/AspectJ/OptimizeRecursionCache.aj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AspectJ".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/AspectJ/CacheAspect.aj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AspectJ".to_string());
        assert_eq!(actual, expected);
    }
}

mod assembly {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Assembly/macros.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Assembly/External Interrupt.a51");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Assembly/FASM.asm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Assembly/forth.nasm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Assembly/lib.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Assembly/A8514.I");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Assembly/3D_PRG.I");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Assembly/audio.i");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Assembly/fp_sqr32_160_comba.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Assembly".to_string());
        assert_eq!(actual, expected);
    }
}

mod astro {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Astro/index.astro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Astro".to_string());
        assert_eq!(actual, expected);
    }
}

mod asymptote {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Asymptote/kappa-sawteeth.asy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Asymptote".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Asymptote/figarc4_3D.asy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Asymptote".to_string());
        assert_eq!(actual, expected);
    }
}

mod autohotkey {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/AutoHotkey/hello.ahk");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("AutoHotkey".to_string());
        assert_eq!(actual, expected);
    }
}

mod avro_idl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Avro IDL/user.avdl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Avro IDL".to_string());
        assert_eq!(actual, expected);
    }
}

mod awk {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Awk/test.awk");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Awk".to_string());
        assert_eq!(actual, expected);
    }
}

mod basic {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/BASIC/mandelbrot.bas");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("BASIC".to_string());
        assert_eq!(actual, expected);
    }
}

mod ballerina {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Ballerina/hello-world-service.bal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ballerina".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Ballerina/hello-world.bal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ballerina".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Ballerina/var.bal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ballerina".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Ballerina/json.bal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ballerina".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Ballerina/xml.bal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ballerina".to_string());
        assert_eq!(actual, expected);
    }
}

mod beef {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Beef/RandoCode.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Beef".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Beef/ProfilePanel.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Beef".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Beef/Program.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Beef".to_string());
        assert_eq!(actual, expected);
    }
}

mod bibtex {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/BibTeX/deeplyaggrevated.bibtex");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("BibTeX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/BibTeX/citations.bib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("BibTeX".to_string());
        assert_eq!(actual, expected);
    }
}

mod bicep {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Bicep/101-container-registry.bicep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Bicep".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Bicep/201-1vm-2nics-2subnets-1vnet.bicep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Bicep".to_string());
        assert_eq!(actual, expected);
    }
}

mod bitbake {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/BitBake/gstreamer-libav.bb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("BitBake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/BitBake/qtbase-native.bb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("BitBake".to_string());
        assert_eq!(actual, expected);
    }
}

mod blade {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Blade/hello.blade");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Blade".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Blade/hello.blade.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Blade".to_string());
        assert_eq!(actual, expected);
    }
}

mod blitzbasic {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/BlitzBasic/PObj.bb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("BlitzBasic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/BlitzBasic/LList.bb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("BlitzBasic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/BlitzBasic/HalfAndDouble.bb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("BlitzBasic".to_string());
        assert_eq!(actual, expected);
    }
}

mod blitzmax {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/BlitzMax/sample.bmx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("BlitzMax".to_string());
        assert_eq!(actual, expected);
    }
}

mod bluespec {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Bluespec/TbTL.bsv");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Bluespec".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Bluespec/TL.bsv");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Bluespec".to_string());
        assert_eq!(actual, expected);
    }
}

mod boogie {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Boogie/TuringFactorial.bpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Boogie".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Boogie/ticket.bpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Boogie".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Boogie/Bubble.bpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Boogie".to_string());
        assert_eq!(actual, expected);
    }
}

mod brainfuck {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Brainfuck/hello.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Brainfuck".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Brainfuck/factor.b");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Brainfuck".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Brainfuck/rot13.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Brainfuck".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Brainfuck/helloworld.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Brainfuck".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Brainfuck/fib100.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Brainfuck".to_string());
        assert_eq!(actual, expected);
    }
}

mod brightscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Brightscript/SimpleGrid.brs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Brightscript".to_string());
        assert_eq!(actual, expected);
    }
}

mod browserslist {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Browserslist/filenames/.browserslistrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Browserslist".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Browserslist/filenames/browserslist");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Browserslist".to_string());
        assert_eq!(actual, expected);
    }
}

mod c {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/C/cpuid.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/C/NWMan.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/C/http_parser.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/C/rdiscount.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/C/hello.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/C/wglew.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/C/http_parser.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/C/GLKMatrix4.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/C/readline.cats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/C/commit.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/C/pqiv.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/C/syscalls.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/C/elf.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/C/ArrowLeft.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/C/interface.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/C/Nightmare.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/C/info.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/C/Arduino.cats");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/C/scheduler.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/C/vmem.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/C/2D.C");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/C/bootstrap.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/C/rfc_string.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/C/multiboot.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/C/exception.zep.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/C/filter.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/C/hello.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/C/fudge_node.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/C/rf_io.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_29() {
        let path = Path::new(&"./samples/C/bitmap.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_30() {
        let path = Path::new(&"./samples/C/rfc_string.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_31() {
        let path = Path::new(&"./samples/C/rf_io.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_32() {
        let path = Path::new(&"./samples/C/syscalldefs.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_33() {
        let path = Path::new(&"./samples/C/array.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_34() {
        let path = Path::new(&"./samples/C/color.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_35() {
        let path = Path::new(&"./samples/C/sgd_fast.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_36() {
        let path = Path::new(&"./samples/C/git.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_37() {
        let path = Path::new(&"./samples/C/yajl.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_38() {
        let path = Path::new(&"./samples/C/script");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_39() {
        let path = Path::new(&"./samples/C/array.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_40() {
        let path = Path::new(&"./samples/C/Field.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_41() {
        let path = Path::new(&"./samples/C/custom_extensions.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_42() {
        let path = Path::new(&"./samples/C/ip4.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_43() {
        let path = Path::new(&"./samples/C/portio.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_44() {
        let path = Path::new(&"./samples/C/blob.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_45() {
        let path = Path::new(&"./samples/C/blob.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_46() {
        let path = Path::new(&"./samples/C/asm.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_47() {
        let path = Path::new(&"./samples/C/exception.zep.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_48() {
        let path = Path::new(&"./samples/C/rpc.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_49() {
        let path = Path::new(&"./samples/C/redis.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_50() {
        let path = Path::new(&"./samples/C/ntru_encrypt.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_51() {
        let path = Path::new(&"./samples/C/vfs.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_52() {
        let path = Path::new(&"./samples/C/driver.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_53() {
        let path = Path::new(&"./samples/C/jni_layer.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_54() {
        let path = Path::new(&"./samples/C/process.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_55() {
        let path = Path::new(&"./samples/C/2D.H");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_56() {
        let path = Path::new(&"./samples/C/markdown.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_57() {
        let path = Path::new(&"./samples/C/commit.c");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C".to_string());
        assert_eq!(actual, expected);
    }
}

mod c_sharp_ {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/C#/chart-process-memory.linq");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/C#/MongoExpressionVisitor.cs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/C#/build.cake");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/C#/BsonPropertyValue.cs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/C#/AssemblyInfo.cs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/C#/Program.cs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C#".to_string());
        assert_eq!(actual, expected);
    }
}

mod c_plus__plus_ {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/C++/cnokw.re");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/C++/key.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/C++/render_adapter.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/C++/bar.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/C++/simple.re");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/C++/Entity.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/C++/bug1163046.--skeleton.re");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/C++/grpc.pb.cc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/C++/instances.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/C++/epoll_reactor.ipp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/C++/PackageInfoParser.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/C++/hello.ino");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/C++/scanner.cc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[ignore]
    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/C++/initClasses.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/C++/Types.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/C++/hello.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/C++/env.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/C++/gdsdbreader.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/C++/crypter.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/C++/srs_app_ingest.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/C++/protocol-buffer.pb.cc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/C++/scanner.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/C++/json_writer.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/C++/bar.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/C++/v8.cc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/C++/CsvStreamer.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/C++/key.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/C++/metrics.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/C++/json_reader.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_29() {
        let path = Path::new(&"./samples/C++/constexpr_header.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_30() {
        let path = Path::new(&"./samples/C++/cvsignore.re");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_31() {
        let path = Path::new(&"./samples/C++/octave_changer.ino");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_32() {
        let path = Path::new(&"./samples/C++/runtime-compiler.cc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_33() {
        let path = Path::new(&"./samples/C++/bar.hpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_34() {
        let path = Path::new(&"./samples/C++/wrapper_inner.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_35() {
        let path = Path::new(&"./samples/C++/v8.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_36() {
        let path = Path::new(&"./samples/C++/program.cp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_37() {
        let path = Path::new(&"./samples/C++/ThreadedQueue.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_38() {
        let path = Path::new(&"./samples/C++/16F88.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_39() {
        let path = Path::new(&"./samples/C++/utils.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_40() {
        let path = Path::new(&"./samples/C++/protocol-buffer.pb.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_41() {
        let path = Path::new(&"./samples/C++/libcanister.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_42() {
        let path = Path::new(&"./samples/C++/main.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_43() {
        let path = Path::new(&"./samples/C++/env.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_44() {
        let path = Path::new(&"./samples/C++/ClasspathVMSystemProperties.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_45() {
        let path = Path::new(&"./samples/C++/Math.inl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_46() {
        let path = Path::new(&"./samples/C++/graphics.cpp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_47() {
        let path = Path::new(&"./samples/C++/Memory16F88.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_48() {
        let path = Path::new(&"./samples/C++/hello.grpc.pb.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("C++".to_string());
        assert_eq!(actual, expected);
    }
}

mod cil {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CIL/guix-daemon.cil");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CIL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/CIL/certfile.cil");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CIL".to_string());
        assert_eq!(actual, expected);
    }
}

mod clips {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CLIPS/demo.clp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CLIPS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/CLIPS/sudoku.clp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CLIPS".to_string());
        assert_eq!(actual, expected);
    }
}

mod cmake {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CMake/sample4.cmake");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CMake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/CMake/uninstall.cmake.in");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CMake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/CMake/sample3.cmake");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CMake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/CMake/sample5.cmake");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CMake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/CMake/sample2.cmake");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CMake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/CMake/sample1.cmake");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CMake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/CMake/filenames/CMakeLists.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CMake".to_string());
        assert_eq!(actual, expected);
    }
}

mod cobol {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/COBOL/hello_world.cob");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("COBOL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/COBOL/hello_world.cbl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("COBOL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/COBOL/hello_world.ccp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("COBOL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/COBOL/simple.cpy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("COBOL".to_string());
        assert_eq!(actual, expected);
    }
}

mod codeowners {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CODEOWNERS/filenames/CODEOWNERS");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CODEOWNERS".to_string());
        assert_eq!(actual, expected);
    }
}

mod cson {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CSON/base.cson");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/CSON/config.cson");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/CSON/wercker-status.cson");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/CSON/ff-sfd.cson");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CSON".to_string());
        assert_eq!(actual, expected);
    }
}

mod css {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CSS/bootstrap.css");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CSS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/CSS/bootstrap.min.css");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CSS".to_string());
        assert_eq!(actual, expected);
    }
}

mod csv {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CSV/cars.csv");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CSV".to_string());
        assert_eq!(actual, expected);
    }
}

mod cue {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CUE/github-workflow.cue");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CUE".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/CUE/kube.cue");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CUE".to_string());
        assert_eq!(actual, expected);
    }
}

mod cweb {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CWeb/sat-life.w");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CWeb".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/CWeb/mpmathdouble.w");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CWeb".to_string());
        assert_eq!(actual, expected);
    }
}

mod cabal_config {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Cabal Config/line2pdf.cabal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cabal Config".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Cabal Config/defaults.cabal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cabal Config".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Cabal Config/filenames/cabal.config");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cabal Config".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Cabal Config/filenames/cabal.project");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cabal Config".to_string());
        assert_eq!(actual, expected);
    }
}

mod cartocss {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CartoCSS/amenity-points.mss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CartoCSS".to_string());
        assert_eq!(actual, expected);
    }
}

mod ceylon {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Ceylon/Foo.ceylon");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ceylon".to_string());
        assert_eq!(actual, expected);
    }
}

mod chapel {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Chapel/distributions.chpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Chapel".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Chapel/hello.chpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Chapel".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Chapel/lulesh.chpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Chapel".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Chapel/quicksort.chpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Chapel".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Chapel/nbody.chpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Chapel".to_string());
        assert_eq!(actual, expected);
    }
}

mod charity {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Charity/example.ch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Charity".to_string());
        assert_eq!(actual, expected);
    }
}

mod cirru {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Cirru/line.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Cirru/indent.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Cirru/html.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Cirru/folding.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Cirru/folded-beginning.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Cirru/quote.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Cirru/webpack.config.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Cirru/demo.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Cirru/spaces.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Cirru/calcit.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Cirru/parentheses.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Cirru/template.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Cirru/comma.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Cirru/unfolding.cirru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cirru".to_string());
        assert_eq!(actual, expected);
    }
}

mod clarion {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Clarion/hello.clw");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clarion".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Clarion/HelloWorld.clw");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clarion".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Clarion/ConsoleSupport.clw");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clarion".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Clarion/CStringClass.clw");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clarion".to_string());
        assert_eq!(actual, expected);
    }
}

mod classic_asp {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Classic ASP/sendingcontent-xml.asp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Classic ASP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Classic ASP/ASPUnitRunner.asp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Classic ASP".to_string());
        assert_eq!(actual, expected);
    }
}

mod clean {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Clean/stack.icl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clean".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Clean/GenMap.icl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clean".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Clean/GenMap.dcl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clean".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Clean/sem.icl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clean".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Clean/streams.icl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clean".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Clean/stack.dcl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clean".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Clean/GenHylo.dcl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clean".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Clean/streams.dcl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clean".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Clean/fsieve.icl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clean".to_string());
        assert_eq!(actual, expected);
    }
}

mod click {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Click/thomer-nat.click");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Click".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Click/sr2.click");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Click".to_string());
        assert_eq!(actual, expected);
    }
}

mod clojure {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Clojure/rand.cljscm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clojure".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Clojure/hiccup.hic");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clojure".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Clojure/svg.cljx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clojure".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Clojure/for.clj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clojure".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Clojure/index.cljs.hl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clojure".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Clojure/build.boot");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clojure".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Clojure/protocol.cljs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clojure".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Clojure/into-array.cljc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clojure".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Clojure/unit-test.cl2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Clojure".to_string());
        assert_eq!(actual, expected);
    }
}

mod closure_templates {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Closure Templates/example.soy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Closure Templates".to_string());
        assert_eq!(actual, expected);
    }
}

mod cloud_firestore_security_rules {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Cloud Firestore Security Rules/filenames/firestore.rules");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cloud Firestore Security Rules".to_string());
        assert_eq!(actual, expected);
    }
}

mod conll_u {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CoNLL-U/ug-ud-test-abridged.conllu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoNLL-U".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/CoNLL-U/CF1.conllu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoNLL-U".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/CoNLL-U/en-ud-test-abridged.conllu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoNLL-U".to_string());
        assert_eq!(actual, expected);
    }
}

mod codeql {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CodeQL/qll-sample2.qll");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CodeQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/CodeQL/qll-sample3.qll");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CodeQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/CodeQL/ql-sample.ql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CodeQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/CodeQL/qll-sample1.qll");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CodeQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/CodeQL/tree-sitter-sample.ql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CodeQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/CodeQL/ql-cobol-sample.ql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CodeQL".to_string());
        assert_eq!(actual, expected);
    }
}

mod coffeescript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/CoffeeScript/coffee-script.coffee");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/CoffeeScript/browser.coffee");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/CoffeeScript/example.cjsx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/CoffeeScript/intro.coffee");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/CoffeeScript/rack_application.coffee");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/CoffeeScript/classes.coffee");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/CoffeeScript/build.cake");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/CoffeeScript/lexer.coffee");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/CoffeeScript/hello.coffee");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/CoffeeScript/xipd.coffee");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod coldfusion {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ColdFusion/example.cfm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ColdFusion".to_string());
        assert_eq!(actual, expected);
    }
}

mod coldfusion_cfc {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ColdFusion CFC/exampleScript.cfc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ColdFusion CFC".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/ColdFusion CFC/exampleTag.cfc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ColdFusion CFC".to_string());
        assert_eq!(actual, expected);
    }
}

mod common_lisp {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Common Lisp/hello.lisp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Common Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Common Lisp/rss.sexp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Common Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Common Lisp/sample.lsp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Common Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Common Lisp/array.l");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Common Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Common Lisp/common.l");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Common Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Common Lisp/macros-advanced.cl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Common Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Common Lisp/motor-inferencia.cl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Common Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Common Lisp/sample.lisp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Common Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Common Lisp/config.sexp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Common Lisp".to_string());
        assert_eq!(actual, expected);
    }
}

mod common_workflow_language {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Common Workflow Language/trunk-peak-score.cwl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Common Workflow Language".to_string());
        assert_eq!(actual, expected);
    }
}

mod component_pascal {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Component Pascal/Example2.cps");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Component Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Component Pascal/Example.cp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Component Pascal".to_string());
        assert_eq!(actual, expected);
    }
}

mod cool {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Cool/list.cl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cool".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Cool/sample.cl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cool".to_string());
        assert_eq!(actual, expected);
    }
}

mod coq {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Coq/Lists.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Coq/Smallstep.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Coq/Stlc.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Coq/JsInterpreterExtraction.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Coq/Rel.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Coq/JsCorrectness.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Coq/JsNumber.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Coq/Computation.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Coq/Spec.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Coq/JsPrettyInterm.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Coq/Poly.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Coq/Main.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Coq/Imp.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Coq".to_string());
        assert_eq!(actual, expected);
    }
}

mod creole {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Creole/creole.creole");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Creole".to_string());
        assert_eq!(actual, expected);
    }
}

mod crystal {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Crystal/transformer.cr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Crystal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Crystal/declare_var_spec.cr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Crystal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Crystal/const_spec.cr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Crystal".to_string());
        assert_eq!(actual, expected);
    }
}

mod csound {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Csound/interp.orc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Csound".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Csound/test.orc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Csound".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Csound/allglass.orc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Csound".to_string());
        assert_eq!(actual, expected);
    }
}

mod csound_document {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Csound Document/test.csd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Csound Document".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Csound Document/allglass.csd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Csound Document".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Csound Document/interp.csd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Csound Document".to_string());
        assert_eq!(actual, expected);
    }
}

mod csound_score {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Csound Score/test.sco");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Csound Score".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Csound Score/interp.sco");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Csound Score".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Csound Score/allglass.sco");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Csound Score".to_string());
        assert_eq!(actual, expected);
    }
}

mod cuda {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Cuda/scalarProd_kernel.cuh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cuda".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Cuda/vectorAdd.cu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cuda".to_string());
        assert_eq!(actual, expected);
    }
}

mod cue_sheet {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Cue Sheet/sample2.cue");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cue Sheet".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Cue Sheet/sample1.cue");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cue Sheet".to_string());
        assert_eq!(actual, expected);
    }
}

mod cycript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Cycript/utils.cy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Cycript".to_string());
        assert_eq!(actual, expected);
    }
}

mod d {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/D/hello_world.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/D/unittest1.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/D/unittest2.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/D/aa.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/D/template_function.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/D/template.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/D/function.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/D/arrayops.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("D".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/D/mpq.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("D".to_string());
        assert_eq!(actual, expected);
    }
}

mod digital_command_language {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/DIGITAL Command Language/libxslt_build.com");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DIGITAL Command Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/DIGITAL Command Language/ghostpdl_zlib_make_vms.com");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DIGITAL Command Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/DIGITAL Command Language/vmsbackup_build.com");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DIGITAL Command Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/DIGITAL Command Language/fis_gtm_kitinstal.com");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DIGITAL Command Language".to_string());
        assert_eq!(actual, expected);
    }
}

mod dm {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/DM/example.dm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DM".to_string());
        assert_eq!(actual, expected);
    }
}

mod dns_zone {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/DNS Zone/sample.arpa");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DNS Zone".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/DNS Zone/sneaky.net.zone");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DNS Zone".to_string());
        assert_eq!(actual, expected);
    }
}

mod dtrace {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/DTrace/trace_futexes.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DTrace".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/DTrace/probes.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DTrace".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/DTrace/counts.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DTrace".to_string());
        assert_eq!(actual, expected);
    }
}

mod dafny {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Dafny/Io.s.dfy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Dafny".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Dafny/Node.i.dfy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Dafny".to_string());
        assert_eq!(actual, expected);
    }
}

mod dart {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Dart/point.dart");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Dart".to_string());
        assert_eq!(actual, expected);
    }
}

mod dataweave {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/DataWeave/customInterpolator.dwl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DataWeave".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/DataWeave/literals.dwl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DataWeave".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/DataWeave/directives.dwl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DataWeave".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/DataWeave/match.dwl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DataWeave".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/DataWeave/functions.dwl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DataWeave".to_string());
        assert_eq!(actual, expected);
    }
}

mod dhall {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Dhall/largeExpressionA.dhall");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Dhall".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Dhall/remoteSystemsA.dhall");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Dhall".to_string());
        assert_eq!(actual, expected);
    }
}

mod diff {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Diff/dude-thing-okay--001.patch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Diff".to_string());
        assert_eq!(actual, expected);
    }
}

mod directx_3d_file {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/DirectX 3D File/cube.x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("DirectX 3D File".to_string());
        assert_eq!(actual, expected);
    }
}

mod dockerfile {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Dockerfile/filenames/Dockerfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Dockerfile".to_string());
        assert_eq!(actual, expected);
    }
}

mod dogescript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Dogescript/example.djs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Dogescript".to_string());
        assert_eq!(actual, expected);
    }
}

mod e {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/E/Promises.E");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("E".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/E/minChat.E");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("E".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/E/Extends.E");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("E".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/E/Guards.E");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("E".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/E/Functions.E");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("E".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/E/atomic-updates.E");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("E".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/E/IO.E");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("E".to_string());
        assert_eq!(actual, expected);
    }
}

mod e_mail {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/E-mail/example.eml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("E-mail".to_string());
        assert_eq!(actual, expected);
    }
}

mod ebnf {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/EBNF/object.ebnf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EBNF".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/EBNF/types.ebnf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EBNF".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/EBNF/grammar.ebnf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EBNF".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/EBNF/material.ebnf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EBNF".to_string());
        assert_eq!(actual, expected);
    }
}

mod ecl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ECL/sample.ecl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ECL".to_string());
        assert_eq!(actual, expected);
    }
}

mod eclipse {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ECLiPSe/or-constraint.ecl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ECLiPSe".to_string());
        assert_eq!(actual, expected);
    }
}

mod ejs {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/EJS/email_template.ect");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EJS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/EJS/modules.ejs.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EJS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/EJS/admin_index.jst");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EJS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/EJS/dash.ejs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EJS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/EJS/page.ejs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EJS".to_string());
        assert_eq!(actual, expected);
    }
}

mod eq {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/EQ/SEButtonEntity.eq");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EQ".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/EQ/String.eq");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EQ".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/EQ/HTTPServerVirtualHostListener.eq");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EQ".to_string());
        assert_eq!(actual, expected);
    }
}

mod eagle {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Eagle/Eagle.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Eagle".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Eagle/Eagle.brd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Eagle".to_string());
        assert_eq!(actual, expected);
    }
}

mod earthly {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Earthly/filenames/Earthfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Earthly".to_string());
        assert_eq!(actual, expected);
    }
}

mod easybuild {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Easybuild/bzip2-1.0.6-GCC-4.9.2.eb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Easybuild".to_string());
        assert_eq!(actual, expected);
    }
}

mod editorconfig {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/EditorConfig/filenames/.editorconfig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EditorConfig".to_string());
        assert_eq!(actual, expected);
    }
}

mod edje_data_collection {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Edje Data Collection/mild.edc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Edje Data Collection".to_string());
        assert_eq!(actual, expected);
    }
}

mod eiffel {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Eiffel/application.e");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Eiffel".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Eiffel/git_checkout_command.e");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Eiffel".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Eiffel/book_collection.e");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Eiffel".to_string());
        assert_eq!(actual, expected);
    }
}

mod elixir {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Elixir/filenames/mix.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Elixir".to_string());
        assert_eq!(actual, expected);
    }
}

mod elm {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Elm/Basic.elm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Elm".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Elm/Tree.elm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Elm".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Elm/QuickSort.elm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Elm".to_string());
        assert_eq!(actual, expected);
    }
}

mod emacs_lisp {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Emacs Lisp/ess-julia.el");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Emacs Lisp/.emacs.desktop");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Emacs Lisp/dude.el");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Emacs Lisp/filenames/Cask");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Emacs Lisp/filenames/.viper");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Emacs Lisp/filenames/.spacemacs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Emacs Lisp/filenames/abbrev_defs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Emacs Lisp/filenames/_emacs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Emacs Lisp/filenames/Project.ede");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Emacs Lisp/filenames/.abbrev_defs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Emacs Lisp/filenames/.gnus");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Emacs Lisp".to_string());
        assert_eq!(actual, expected);
    }
}

mod emberscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/EmberScript/momentComponent.em");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("EmberScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod erlang {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Erlang/sample.app.src");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Erlang/record_utils.erl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Erlang/record_helper.erl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Erlang/hello.escript");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Erlang/elixir_parser.yrl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Erlang/release");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Erlang/single-context.es");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Erlang/170-os-daemons.es");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Erlang/factorial");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Erlang/lfe_scan.xrl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Erlang/filenames/rebar.config.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Erlang/filenames/Emakefile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Erlang/filenames/rebar.config");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Erlang/filenames/rebar.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Erlang".to_string());
        assert_eq!(actual, expected);
    }
}

mod f_sharp_ {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/F#/PerformanceTesters.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("F#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/F#/JsonFormat.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("F#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/F#/sample.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("F#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/F#/JsonReader.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("F#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/F#/JsonSerializer.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("F#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/F#/JsonWriter.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("F#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/F#/PerformanceTests.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("F#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/F#/Combinators.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("F#".to_string());
        assert_eq!(actual, expected);
    }
}

mod figlet_font {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/FIGlet Font/ivrit.flf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("FIGlet Font".to_string());
        assert_eq!(actual, expected);
    }
}

mod flux {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/FLUX/gameserver.fx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("FLUX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/FLUX/test.fx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("FLUX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/FLUX/mbittorrent.fx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("FLUX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/FLUX/imageserver.fx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("FLUX".to_string());
        assert_eq!(actual, expected);
    }
}

mod fantom {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Fantom/sample2.fan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fantom".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Fantom/sample1.fan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fantom".to_string());
        assert_eq!(actual, expected);
    }
}

mod faust {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Faust/FFT.dsp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Faust".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Faust/lowCut.dsp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Faust".to_string());
        assert_eq!(actual, expected);
    }
}

mod fennel {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Fennel/repl.fnl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fennel".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Fennel/utils.fnl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fennel".to_string());
        assert_eq!(actual, expected);
    }
}

mod filebench_wml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Filebench WML/copyfiles.f");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Filebench WML".to_string());
        assert_eq!(actual, expected);
    }
}

mod filterscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Filterscript/colormatrix.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Filterscript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Filterscript/fs_kernel.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Filterscript".to_string());
        assert_eq!(actual, expected);
    }
}

mod fluent {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Fluent/simple.ftl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fluent".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Fluent/call_expressions.ftl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fluent".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Fluent/gecko_strings.ftl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fluent".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Fluent/select_indent.ftl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fluent".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Fluent/callee_expressions.ftl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fluent".to_string());
        assert_eq!(actual, expected);
    }
}

mod formatted {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Formatted/NiAlH_jea.eam.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Formatted".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Formatted/long_seq.for");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Formatted".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Formatted/wksst8110.for");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Formatted".to_string());
        assert_eq!(actual, expected);
    }
}

mod forth {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Forth/core.for");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Forth/asm.fr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Forth/core.fth");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Forth/core.f");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Forth/enum.frt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Forth/tools.fth");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Forth/block.fth");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Forth/KataDiversion.fth");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Forth/hello-forth.forth");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Forth/macros.frt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Forth/core-ext.fth");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Forth/core.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Forth/bitmap.frt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Forth/hello-forth.fth");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Forth/tools.4TH");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Forth/core1.F");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Forth".to_string());
        assert_eq!(actual, expected);
    }
}

mod fortran {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Fortran/sample3.F");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fortran".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Fortran/sample1.for");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fortran".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Fortran/sample2.f");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fortran".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Fortran/bug-185631.f");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fortran".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Fortran/sample1.f");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Fortran".to_string());
        assert_eq!(actual, expected);
    }
}

mod freebasic {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/FreeBasic/try_catch_throw.bas");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("FreeBasic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/FreeBasic/WinGUI template.bi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("FreeBasic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/FreeBasic/Plasma Generation.bas");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("FreeBasic".to_string());
        assert_eq!(actual, expected);
    }
}

mod freemarker {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/FreeMarker/layout.ftl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("FreeMarker".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/FreeMarker/example.ftl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("FreeMarker".to_string());
        assert_eq!(actual, expected);
    }
}

mod frege {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Frege/SwingExamples.fr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Frege".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Frege/Sudoku.fr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Frege".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Frege/Concurrent.fr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Frege".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Frege/CommandLineClock.fr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Frege".to_string());
        assert_eq!(actual, expected);
    }
}

mod fstar {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Fstar/Hacl.HKDF.fst");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("F*".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Fstar/Hacl.Spec.Bignum.Fmul.fst");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("F*".to_string());
        assert_eq!(actual, expected);
    }
}

mod futhark {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Futhark/gaussian_blur.fut");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Futhark".to_string());
        assert_eq!(actual, expected);
    }
}

mod g_code {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/G-code/duettest.g");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("G-code".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/G-code/ghLogo.cnc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("G-code".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/G-code/square.g");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("G-code".to_string());
        assert_eq!(actual, expected);
    }
}

mod gaml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/GAML/luneraysFlu.gaml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/GAML/incrementalGIS.gaml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/GAML/bdiAgents.gaml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/GAML/3dTutorial.gaml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/GAML/roadTraffic.gaml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/GAML/predatorPrey.gaml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAML".to_string());
        assert_eq!(actual, expected);
    }
}

mod gams {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/GAMS/transport.gms");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAMS".to_string());
        assert_eq!(actual, expected);
    }
}

mod gap {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/GAP/PackageInfo.g");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/GAP/example.gi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/GAP/vspc.gi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/GAP/bugfix.tst");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/GAP/Magic.gi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/GAP/Magic.gd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/GAP/vspc.gd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/GAP/factor.tst");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/GAP/example.gd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GAP".to_string());
        assert_eq!(actual, expected);
    }
}

mod gcc_machine_description {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/GCC Machine Description/pdp10.md");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GCC Machine Description".to_string());
        assert_eq!(actual, expected);
    }
}

mod gdb {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/GDB/gdb_lpc17xx_program.gdb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GDB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/GDB/as3.gdbinit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GDB".to_string());
        assert_eq!(actual, expected);
    }
}

mod gdscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/GDScript/pong.gd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GDScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/GDScript/player.gd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GDScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/GDScript/example.gd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GDScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/GDScript/grid.gd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GDScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod gedcom {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/GEDCOM/Royal92.ged");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GEDCOM".to_string());
        assert_eq!(actual, expected);
    }
}

mod glsl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/GLSL/myfragment.frg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/GLSL/gbuffers_textured_lit.fsh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/GLSL/pntriangles.tesc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/GLSL/gbuffers_textured_lit.vsh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/GLSL/SyLens.glsl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/GLSL/miss.rmiss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/GLSL/islandScene.shader");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/GLSL/shader.fp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/GLSL/islandScene.glsl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/GLSL/recurse1.frag");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/GLSL/myvertex.vrx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/GLSL/recurse1.fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/GLSL/SyLens.shader");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/GLSL/pntriangles.tese");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/GLSL/closesthit.rchit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/GLSL/SimpleLighting.gl2.frag");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/GLSL/blend_120.glslf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/GLSL/extrude_normals.gs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GLSL".to_string());
        assert_eq!(actual, expected);
    }
}

mod gn {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/GN/isolate.gni");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/GN/clang.gni");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/GN/BUILD.2.gn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/GN/internal_rules.gni");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/GN/BUILD.gn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/GN/android-rules.gni");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/GN/ios-rules.gni");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/GN/icu.gn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/GN/gcc_toolchain.gni");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/GN/BUILD.3.gn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/GN/filenames/.gn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GN".to_string());
        assert_eq!(actual, expected);
    }
}

mod game_maker_language {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Game Maker Language/_piwikSendReq.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Game Maker Language/faucet-http.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Game Maker Language/jsonion_test.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Game Maker Language/_piwikSendBasicReq.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Game Maker Language/_piwikCacheRequest.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Game Maker Language/characterStepEvent.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Game Maker Language/draw_menu.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Game Maker Language/jsonion.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Game Maker Language/scrInitLevel.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Game Maker Language/characterDrawEvent.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Game Maker Language/GMLmenus.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Game Maker Language".to_string());
        assert_eq!(actual, expected);
    }
}

mod gemfile_dot_lock {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Gemfile.lock/filenames/Gemfile.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gemfile.lock".to_string());
        assert_eq!(actual, expected);
    }
}

mod genie {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Genie/IDataLoader.gs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Genie".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Genie/Class.gs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Genie".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Genie/web.gs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Genie".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Genie/Hello.gs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Genie".to_string());
        assert_eq!(actual, expected);
    }
}

mod gerber_image {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Gerber Image/FelinaePurr-B.Mask.gbs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Gerber Image/AGV_GPIO.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Gerber Image/simonShield-F.Mask.gbr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Gerber Image/simonShield-B.Cu.gbr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Gerber Image/simonShield-F.Cu.gbr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Gerber Image/FelinaePurr-B.Cu.gbl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Gerber Image/simonShield-Edge.Cuts.gbr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Gerber Image/FelinaePurr-F.Paste.gtp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Gerber Image/nonaprs_contour.gko");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Gerber Image/FelinaePurr-F.Mask.gts");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Gerber Image/FelinaePurr-B.SilkS.gbo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Gerber Image/simonShield-B.Mask.gbr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Gerber Image/FelinaePurr-F.Cu.gtl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Gerber Image/simonShield-F.SilkS.gbr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Gerber Image/GOLMain_RevA-B.Paste.gbp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Gerber Image/simonShield-drl_map.gbr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Gerber Image/FelinaePurr-F.SilkS.gto");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Gerber Image/LIDARLite.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gerber Image".to_string());
        assert_eq!(actual, expected);
    }
}

mod gherkin {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Gherkin/tables.feature");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gherkin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Gherkin/resources.story");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gherkin".to_string());
        assert_eq!(actual, expected);
    }
}

mod git_attributes {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Git Attributes/filenames/.gitattributes");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Git Attributes".to_string());
        assert_eq!(actual, expected);
    }
}

mod git_config {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Git Config/aliases.gitconfig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Git Config".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Git Config/filenames/.gitmodules");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Git Config".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Git Config/filenames/.gitconfig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Git Config".to_string());
        assert_eq!(actual, expected);
    }
}

mod glyph_bitmap_distribution_format {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Glyph Bitmap Distribution Format/bitbuntu.bdf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Glyph Bitmap Distribution Format".to_string());
        assert_eq!(actual, expected);
    }
}

mod gnuplot {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Gnuplot/world2.1.gnu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gnuplot".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Gnuplot/histograms.2.gnu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gnuplot".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Gnuplot/surface1.16.gnu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gnuplot".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Gnuplot/rates.gp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gnuplot".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Gnuplot/surface1.17.gnu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gnuplot".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Gnuplot/defense_plotter.p");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gnuplot".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Gnuplot/dashcolor.1.gnu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gnuplot".to_string());
        assert_eq!(actual, expected);
    }
}

mod go {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Go/embedded.go");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Go".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Go/oapi-codegen.go");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Go".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Go/api.pb.go");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Go".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Go/gen-go-linguist-thrift.go");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Go".to_string());
        assert_eq!(actual, expected);
    }
}

mod go_checksums {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Go Checksums/filenames/go.sum");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Go Checksums".to_string());
        assert_eq!(actual, expected);
    }
}

mod go_module {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Go Module/filenames/go.mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Go Module".to_string());
        assert_eq!(actual, expected);
    }
}

mod golo {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Golo/workers.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Golo/structs.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Golo/dynamic-evaluation.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Golo/enums-thread-state.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Golo/dynamic-object-person.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Golo/prepost-decorators.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Golo/fibonacci.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Golo/max-int.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Golo/swing-actionlistener.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Golo/util-containers.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Golo/closures.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Golo/http-server.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Golo/decorators.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Golo/adapters.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Golo/matching-operator.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Golo/augmentations.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Golo/swing-helloworld.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Golo/null-safety.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Golo/templates-chat-webapp.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Golo/memoize.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Golo/collection-literals.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Golo/echo-args.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Golo/context-decorator.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/Golo/logdeco.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/Golo/coin-change.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/Golo/async.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/Golo/helloworld.golo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Golo".to_string());
        assert_eq!(actual, expected);
    }
}

mod gosu {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Gosu/Hello.gsx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gosu".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Gosu/Person.gs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gosu".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Gosu/Ronin.gs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gosu".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Gosu/hello.vark");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gosu".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Gosu/Hello.gst");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gosu".to_string());
        assert_eq!(actual, expected);
    }
}

mod grace {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Grace/ackerman_function.grace");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grace".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Grace/grace_IDE.grace");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grace".to_string());
        assert_eq!(actual, expected);
    }
}

mod gradle {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Gradle/builder.gradle");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gradle".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Gradle/build.gradle");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Gradle".to_string());
        assert_eq!(actual, expected);
    }
}

mod grammatical_framework {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Grammatical Framework/LexFoodsSwe.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsEpo.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsPes.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsTur.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Grammatical Framework/LexFoodsGer.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsRon.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsUrd.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsIce.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsHeb.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsPor.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsIta.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsSpa.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsCat.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsChi.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Grammatical Framework/ResCze.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsNep.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Grammatical Framework/LexFoodsIta.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsTha.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsOri.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsAmh.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsFre.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsFin.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsHin.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsJpn.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/Grammatical Framework/LexFoodsCat.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsBul.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsI.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsAfr.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsGer.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_29() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsEng.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_30() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsCze.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_31() {
        let path = Path::new(&"./samples/Grammatical Framework/LexFoods.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_32() {
        let path = Path::new(&"./samples/Grammatical Framework/LexFoodsFin.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_33() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsMon.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_34() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsDut.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_35() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsMlt.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_36() {
        let path = Path::new(&"./samples/Grammatical Framework/Foods.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_37() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsSwe.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_38() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsTsn.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_39() {
        let path = Path::new(&"./samples/Grammatical Framework/FoodsLav.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_40() {
        let path = Path::new(&"./samples/Grammatical Framework/transFoodsHin.gf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Grammatical Framework".to_string());
        assert_eq!(actual, expected);
    }
}

mod graph_modeling_language {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Graph Modeling Language/sample3.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Graph Modeling Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Graph Modeling Language/sample.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Graph Modeling Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Graph Modeling Language/sample2.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Graph Modeling Language".to_string());
        assert_eq!(actual, expected);
    }
}

mod graphql {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/GraphQL/schema-kitchen-sink.graphqls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GraphQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/GraphQL/schema-kitchen-sink.graphql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GraphQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/GraphQL/schema.graphqls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GraphQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/GraphQL/kitchen-sink.graphql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("GraphQL".to_string());
        assert_eq!(actual, expected);
    }
}

mod graphviz__dot_ {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Graphviz (DOT)/annoying.DOT");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Graphviz (DOT)".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Graphviz (DOT)/sample.dot");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Graphviz (DOT)".to_string());
        assert_eq!(actual, expected);
    }
}

mod groovy {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Groovy/template.gtpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Groovy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Groovy/script.gvy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Groovy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Groovy/groovy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Groovy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Groovy/template.grt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Groovy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Groovy/build.gvy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Groovy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Groovy/filenames/Jenkinsfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Groovy".to_string());
        assert_eq!(actual, expected);
    }
}

mod groovy_server_pages {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Groovy Server Pages/hello-pagedirective.gsp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Groovy Server Pages".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Groovy Server Pages/hello-var.gsp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Groovy Server Pages".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Groovy Server Pages/hello-resources.gsp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Groovy Server Pages".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Groovy Server Pages/bar.gsp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Groovy Server Pages".to_string());
        assert_eq!(actual, expected);
    }
}

mod haproxy {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HAProxy/haproxy.cfg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HAProxy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HAProxy/haproxy4.cfg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HAProxy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/HAProxy/haproxy2.cfg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HAProxy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/HAProxy/haproxy3.cfg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HAProxy".to_string());
        assert_eq!(actual, expected);
    }
}

mod hcl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HCL/example.hcl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HCL/main.tf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/HCL/example.nomad");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/HCL/example.tf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/HCL/terraform.tfvars");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/HCL/main.workflow");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HCL".to_string());
        assert_eq!(actual, expected);
    }
}

mod hlsl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HLSL/noise.fx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HLSL/corridor.fx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/HLSL/jellyfish.fx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/HLSL/bloom.cginc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HLSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/HLSL/accelerated_surface_win.hlsl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HLSL".to_string());
        assert_eq!(actual, expected);
    }
}

mod html {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HTML/pages.html");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HTML/pkgdown.html");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/HTML/wehaveoddjobs.hta");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/HTML/example.xht");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/HTML/tailDel.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/HTML/index.html.hl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/HTML/Crear_logo.hta");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/HTML/rpanel.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML".to_string());
        assert_eq!(actual, expected);
    }
}

mod html_plus_ecr {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HTML+ECR/greeting.ecr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML+ECR".to_string());
        assert_eq!(actual, expected);
    }
}

mod html_plus_eex {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HTML+EEX/live_component.html.leex");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML+EEX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HTML+EEX/index.html.eex");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML+EEX".to_string());
        assert_eq!(actual, expected);
    }
}

mod html_plus_erb {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HTML+ERB/fishbowl.html.erb.deface");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML+ERB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HTML+ERB/index.html.erb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML+ERB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/HTML+ERB/index.rhtml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML+ERB".to_string());
        assert_eq!(actual, expected);
    }
}

mod html_plus_razor {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HTML+Razor/Index.razor");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML+Razor".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HTML+Razor/Index.cshtml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HTML+Razor".to_string());
        assert_eq!(actual, expected);
    }
}

mod hxml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HXML/checkstyle.hxml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HXML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HXML/vshaxe.hxml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HXML".to_string());
        assert_eq!(actual, expected);
    }
}

mod hack {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Hack/UserIDRecipe.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Hack/funs.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Hack/MySecureRequest.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Hack/FakeDB.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Hack/StandardPage.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Hack/phpfile.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Hack/NonStrictFile.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Hack/Controller.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Hack/Nav.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Hack/RecipeWithDemo.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Hack/DBResultRecipe.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Hack/Map.hhi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Hack/GetController.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Hack/HomeController.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Hack/startup.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Hack/funs.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Hack/UnescapedString.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Hack/first.hack");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Hack/UsingUserID.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Hack/index.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Hack/GetAndPostRecipe.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Hack/Request.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Hack/UnescapedStringRecipe.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/Hack/Recipe.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/Hack/StrictFile.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/Hack/error.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/Hack/Documentation.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/Hack/AssertRecipe.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/Hack/Assert.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_29() {
        let path = Path::new(&"./samples/Hack/UserID.hh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hack".to_string());
        assert_eq!(actual, expected);
    }
}

mod haml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Haml/buttons.html.haml.deface");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Haml".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Haml/hello.haml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Haml".to_string());
        assert_eq!(actual, expected);
    }
}

mod handlebars {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Handlebars/each.hbs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Handlebars".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Handlebars/basic.handlebars");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Handlebars".to_string());
        assert_eq!(actual, expected);
    }
}

mod haskell {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Haskell/HsColour.hs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Haskell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Haskell/Main.hs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Haskell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Haskell/maze-solving.hs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Haskell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Haskell/Sudoku.hs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Haskell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Haskell/Hello.hs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Haskell".to_string());
        assert_eq!(actual, expected);
    }
}

mod hiveql {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HiveQL/mi.q");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HiveQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HiveQL/query.hql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HiveQL".to_string());
        assert_eq!(actual, expected);
    }
}

mod holyc {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HolyC/OnceDemo.HC");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HolyC".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HolyC/Spy.HC");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HolyC".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/HolyC/Prompt.HC");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HolyC".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/HolyC/GrAsm.HC");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HolyC".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/HolyC/RandDemo.HC");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HolyC".to_string());
        assert_eq!(actual, expected);
    }
}

mod hy {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Hy/fibonacci.hy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Hy/fizzbuzz");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Hy/hello-world.hy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Hy".to_string());
        assert_eq!(actual, expected);
    }
}

mod hyphy {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/HyPhy/hyphy_cmds.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HyPhy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/HyPhy/MolecularClock.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HyPhy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/HyPhy/profile_test.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HyPhy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/HyPhy/AAModelComparison.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HyPhy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/HyPhy/MFPositiveSelection.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HyPhy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/HyPhy/CodonModelCompare.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HyPhy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/HyPhy/MatrixIndexing.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HyPhy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/HyPhy/dNdSDistributionComparison.bf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("HyPhy".to_string());
        assert_eq!(actual, expected);
    }
}

mod idl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/IDL/mg_acosh.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("IDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/IDL/mg_gcd.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("IDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/IDL/mg_analysis.dlm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("IDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/IDL/mg_trunc.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("IDL".to_string());
        assert_eq!(actual, expected);
    }
}

mod igor_pro {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/IGOR Pro/CodeBrowser.ipf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("IGOR Pro".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/IGOR Pro/functions.ipf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("IGOR Pro".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/IGOR Pro/generic.ipf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("IGOR Pro".to_string());
        assert_eq!(actual, expected);
    }
}

mod ini {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/INI/MouseKeyboard.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("INI".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/INI/spyder_website.lektorproject");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("INI".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/INI/TOVR.dof");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("INI".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/INI/ms.cfg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("INI".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/INI/defaults.properties");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("INI".to_string());
        assert_eq!(actual, expected);
    }

    #[ignore]
    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/INI/ms.properties");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("INI".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/INI/ultimate-temp-controller.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("INI".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/INI/filenames/buildozer.spec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("INI".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/INI/filenames/.flake8");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("INI".to_string());
        assert_eq!(actual, expected);
    }
}

mod idris {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Idris/Chars.idr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Idris".to_string());
        assert_eq!(actual, expected);
    }
}

mod ignore_list {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Ignore List/Finale.gitignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Ignore List/filenames/.npmignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Ignore List/filenames/.cvsignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Ignore List/filenames/.bzrignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Ignore List/filenames/gitignore_global");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Ignore List/filenames/.vscodeignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Ignore List/filenames/.markdownlintignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Ignore List/filenames/.eslintignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Ignore List/filenames/.dockerignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Ignore List/filenames/.coffeelintignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Ignore List/filenames/.gitignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Ignore List/filenames/.babelignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Ignore List/filenames/.eleventyignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Ignore List/filenames/gitignore-global");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Ignore List/filenames/.prettierignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Ignore List/filenames/.nodemonignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Ignore List/filenames/.atomignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Ignore List/filenames/.stylelintignore");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ignore List".to_string());
        assert_eq!(actual, expected);
    }
}

mod imagej_macro {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path =
            Path::new(&"./samples/ImageJ Macro/batch of ratiometric FRET using IO settings.ijm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ImageJ Macro".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/ImageJ Macro/simple.ijm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ImageJ Macro".to_string());
        assert_eq!(actual, expected);
    }
}

mod inform_7 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Inform 7/Trivial Extension.i7x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Inform 7".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Inform 7/story.ni");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Inform 7".to_string());
        assert_eq!(actual, expected);
    }
}

mod inno_setup {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Inno Setup/expat.iss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Inno Setup".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Inno Setup/Default.isl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Inno Setup".to_string());
        assert_eq!(actual, expected);
    }
}

mod ioke {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Ioke/hello.ik");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ioke".to_string());
        assert_eq!(actual, expected);
    }
}

mod isabelle {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Isabelle/HelloWorld.thy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Isabelle".to_string());
        assert_eq!(actual, expected);
    }
}

mod isabelle_root {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Isabelle ROOT/filenames/ROOT");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Isabelle ROOT".to_string());
        assert_eq!(actual, expected);
    }
}

mod j {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/J/hashbang");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("J".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/J/stwij.ijs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("J".to_string());
        assert_eq!(actual, expected);
    }
}

mod jar_manifest {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/JAR Manifest/filenames/MANIFEST.MF");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JAR Manifest".to_string());
        assert_eq!(actual, expected);
    }
}

mod jflex {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/JFlex/LexScan.flex");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JFlex".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/JFlex/java.jflex");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JFlex".to_string());
        assert_eq!(actual, expected);
    }
}

mod json {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/JSON/manifest.webmanifest");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/JSON/recording.har");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/JSON/person.json");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/JSON/block-sync-counter8.ice");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/JSON/geo.geojson");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/JSON/switzerland.topojson");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/JSON/Git Commit.JSON-tmLanguage");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/JSON/2ea73365-b6f1-4bd1-a454-d57a67e50684.yy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/JSON/product.json");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/JSON/manifest.webapp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/JSON/terraform.tfstate.backup");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/JSON/small.tfstate");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/JSON/Material_Alpha_01.gltf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/JSON/GMS2_Project.yyp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/JSON/http_response.avsc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/JSON/schema.json");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/JSON/pack.mcmeta");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/JSON/filenames/.tern-project");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/JSON/filenames/.arcconfig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/JSON/filenames/mcmod.info");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/JSON/filenames/.tern-config");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/JSON/filenames/composer.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/JSON/filenames/.htmlhintrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/JSON/filenames/Pipfile.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/JSON/filenames/.watchmanconfig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/JSON/filenames/.imgbotconfig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON".to_string());
        assert_eq!(actual, expected);
    }
}

mod json_with_comments {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/JSON with Comments/Dart.sublime-commands");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/JSON with Comments/CLIPS.sublime-settings");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/JSON with Comments/coc.jsonc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/JSON with Comments/SourcePawn.sublime-build");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/JSON with Comments/Context.sublime-menu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/JSON with Comments/Default.sublime-keymap");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/JSON with Comments/Tubnil.sublime-theme");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/JSON with Comments/AMPL.sublime-build");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/JSON with Comments/Dart.sublime-project");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/JSON with Comments/JavaDoc Add Line.sublime-macro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/JSON with Comments/RunBuild.sublime-macro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/JSON with Comments/jade.sublime-completions");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/JSON with Comments/Default (Linux).sublime-mousemap");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/JSON with Comments/filenames/.jscsrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/JSON with Comments/filenames/tslint.json");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/JSON with Comments/filenames/.eslintrc.json");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/JSON with Comments/filenames/devcontainer.json");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/JSON with Comments/filenames/language-configuration.json");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/JSON with Comments/filenames/.jslintrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/JSON with Comments/filenames/tsconfig.json");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/JSON with Comments/filenames/.babelrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/JSON with Comments/filenames/.jshintrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/JSON with Comments/filenames/jsconfig.json");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON with Comments".to_string());
        assert_eq!(actual, expected);
    }
}

mod json5 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/JSON5/example.json5");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON5".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/JSON5/package.json5");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSON5".to_string());
        assert_eq!(actual, expected);
    }
}

mod jsonld {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/JSONLD/sample.jsonld");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSONLD".to_string());
        assert_eq!(actual, expected);
    }
}

mod jsoniq {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/JSONiq/query.jq");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSONiq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/JSONiq/detail.jq");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JSONiq".to_string());
        assert_eq!(actual, expected);
    }
}

mod jasmin {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Jasmin/op2.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jasmin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Jasmin/if3.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jasmin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Jasmin/op3.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jasmin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Jasmin/if2.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jasmin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Jasmin/op1.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jasmin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Jasmin/if1.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jasmin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Jasmin/if4.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jasmin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Jasmin/op4.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jasmin".to_string());
        assert_eq!(actual, expected);
    }
}

mod java {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Java/Hudson.java");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Java/NokogiriService.java");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Java/clojure-type.java");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Java/HtmlDomParserContext.java");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Java/gen-java-linguist-thrift.java");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Java/ProtocolBuffer.java");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Java/generated-jooq-table.java");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Java/clojure-util.java");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Java/JFlexLexer.java");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Java/GrammarKit.java");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java".to_string());
        assert_eq!(actual, expected);
    }
}

mod java_properties {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[ignore]
    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Java Properties/libraries.properties");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java Properties".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Java Properties/sounds.properties");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Java Properties".to_string());
        assert_eq!(actual, expected);
    }
}

mod javascript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/JavaScript/jquery-1.4.2.min.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/JavaScript/json2_backbone.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/JavaScript/constant_fold.mjs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/JavaScript/helloHanaEndpoint.xsjs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/JavaScript/parser.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/JavaScript/classes.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/JavaScript/gen-js-linguist-thrift.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/JavaScript/intro-old.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/JavaScript/module.mjs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/JavaScript/sample.jsx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/JavaScript/intro.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/JavaScript/http.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/JavaScript/proto.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/JavaScript/intro.js.frag");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/JavaScript/jquery-1.6.1.min.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/JavaScript/outro.js.frag");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/JavaScript/jquery-1.6.1.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/JavaScript/js2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/JavaScript/steelseries-min.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/JavaScript/logo.jscad");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/JavaScript/uglify.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/JavaScript/axios.es");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/JavaScript/entry.mjs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[ignore]
    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/JavaScript/merge.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/JavaScript/namespace.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/JavaScript/run");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/JavaScript/bootstrap-modal.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/JavaScript/js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/JavaScript/hello.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_29() {
        let path = Path::new(&"./samples/JavaScript/dude.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_30() {
        let path = Path::new(&"./samples/JavaScript/jsbuild.jsb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_31() {
        let path = Path::new(&"./samples/JavaScript/classes-old.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_32() {
        let path = Path::new(&"./samples/JavaScript/helloHanaMath.xsjslib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_33() {
        let path = Path::new(&"./samples/JavaScript/chart_composers.gs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_34() {
        let path = Path::new(&"./samples/JavaScript/ccalc-parse.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_35() {
        let path = Path::new(&"./samples/JavaScript/itau.gs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_36() {
        let path = Path::new(&"./samples/JavaScript/index.es");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_37() {
        let path = Path::new(&"./samples/JavaScript/jquery-1.7.2.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_38() {
        let path = Path::new(&"./samples/JavaScript/modernizr.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_39() {
        let path = Path::new(&"./samples/JavaScript/ccalc-lex.js");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod javascript_plus_erb {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/JavaScript+ERB/create.js.erb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("JavaScript+ERB".to_string());
        assert_eq!(actual, expected);
    }
}

mod jest_snapshot {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Jest Snapshot/css.test.tsx.snap");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jest Snapshot".to_string());
        assert_eq!(actual, expected);
    }
}

mod jinja {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Jinja/worker.jinja2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jinja".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Jinja/home.j2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jinja".to_string());
        assert_eq!(actual, expected);
    }
}

mod jison {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Jison/classy.jison");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jison".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Jison/lex.jison");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jison".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Jison/ansic.jison");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jison".to_string());
        assert_eq!(actual, expected);
    }
}

mod jison_lex {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Jison Lex/lex_grammar.jisonlex");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jison Lex".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Jison Lex/classy.jisonlex");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jison Lex".to_string());
        assert_eq!(actual, expected);
    }
}

mod jolie {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Jolie/common.iol");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jolie".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Jolie/student.ol");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jolie".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Jolie/exam.ol");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jolie".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Jolie/examiner.ol");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jolie".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Jolie/hanoi.ol");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jolie".to_string());
        assert_eq!(actual, expected);
    }
}

mod jsonnet {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Jsonnet/intersection.jsonnet");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jsonnet".to_string());
        assert_eq!(actual, expected);
    }
}

mod julia {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Julia/julia");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Julia".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Julia/stockcorr.jl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Julia".to_string());
        assert_eq!(actual, expected);
    }
}

mod jupyter_notebook {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Jupyter Notebook/JupyterNotebook.ipynb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Jupyter Notebook".to_string());
        assert_eq!(actual, expected);
    }
}

mod krl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/KRL/helloworld.krl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KRL".to_string());
        assert_eq!(actual, expected);
    }
}

mod kaitai_struct {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Kaitai Struct/zip.ksy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Kaitai Struct".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Kaitai Struct/iso9660.ksy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Kaitai Struct".to_string());
        assert_eq!(actual, expected);
    }
}

mod kakounescript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/KakouneScript/c-family.kak");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KakouneScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/KakouneScript/doc.kak");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KakouneScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/KakouneScript/filenames/kakrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KakouneScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod kicad_layout {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/KiCad Layout/nrf-bga.kicad_pcb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path =
            Path::new(&"./samples/KiCad Layout/MagneticBuzzer_ProSignal_ABT-410-RC.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/KiCad Layout/Conn_Poncho_SinBorde.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/KiCad Layout/kivicad.kicad_wks");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/KiCad Layout/Logo_OSHWA.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/KiCad Layout/Pin_Header_Straight_2x02.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/KiCad Layout/SW_PUSH_SMALL.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/KiCad Layout/TO-92_Molded_Narrow.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/KiCad Layout/Resistor_Horizontal_RM7mm.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/KiCad Layout/LED-5MM.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/KiCad Layout/C_Disc_D3_P2.5.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/KiCad Layout/Pin_Header_Straight_2x20.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/KiCad Layout/Logo_Poncho.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/KiCad Layout/Fiducial_1mm.kicad_mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/KiCad Layout/simonShield.kicad_pcb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/KiCad Layout/filenames/fp-lib-table");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Layout".to_string());
        assert_eq!(actual, expected);
    }
}

mod kicad_legacy_layout {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/KiCad Legacy Layout/tc14badge.brd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Legacy Layout".to_string());
        assert_eq!(actual, expected);
    }
}

mod kicad_schematic {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/KiCad Schematic/gedda-junk.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Schematic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/KiCad Schematic/ciaaConector.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Schematic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/KiCad Schematic/Volume.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Schematic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/KiCad Schematic/buttons.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Schematic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/KiCad Schematic/buzzer.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Schematic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/KiCad Schematic/ultimate-temp-controller.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("KiCad Schematic".to_string());
        assert_eq!(actual, expected);
    }
}

mod kit {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Kit/demo.kit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Kit".to_string());
        assert_eq!(actual, expected);
    }
}

mod kotlin {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Kotlin/Foo.kt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Kotlin".to_string());
        assert_eq!(actual, expected);
    }
}

mod kusto {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Kusto/kusto-example-queries.csl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Kusto".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Kusto/example-create-sflogs-table.csl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Kusto".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Kusto/examples_traces.csl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Kusto".to_string());
        assert_eq!(actual, expected);
    }
}

mod lfe {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/LFE/mnesia_demo.lfe");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LFE".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/LFE/gps1.lfe");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LFE".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/LFE/church.lfe");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LFE".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/LFE/object.lfe");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LFE".to_string());
        assert_eq!(actual, expected);
    }
}

mod lolcode {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/LOLCODE/LOLTracer.lol");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LOLCODE".to_string());
        assert_eq!(actual, expected);
    }
}

mod lsl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/LSL/LSL.lslp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LSL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/LSL/LSL.lsl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LSL".to_string());
        assert_eq!(actual, expected);
    }
}

mod ltspice_symbol {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/LTspice Symbol/random-shapes.asy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LTspice Symbol".to_string());
        assert_eq!(actual, expected);
    }
}

mod labview {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/LabVIEW/Malleable VIs Basics.lvproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LabVIEW".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/LabVIEW/Malleable VIs - Nested Malleable VIs.lvproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LabVIEW".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/LabVIEW/Temperature Monitoring.lvproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LabVIEW".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/LabVIEW/Work Dispatcher.lvlib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LabVIEW".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/LabVIEW/Person.lvlib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LabVIEW".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/LabVIEW/Coffee Shop.lvlib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LabVIEW".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/LabVIEW/Worker.lvlib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LabVIEW".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/LabVIEW/Customer.lvlib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LabVIEW".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/LabVIEW/Word Finder.lvproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LabVIEW".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/LabVIEW/Actor Framework Fundamentals.lvproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LabVIEW".to_string());
        assert_eq!(actual, expected);
    }
}

mod lark {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Lark/ebl_atf_common.lark");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lark".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Lark/ebl_atf.lark");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lark".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Lark/lark.lark");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lark".to_string());
        assert_eq!(actual, expected);
    }
}

mod lasso {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Lasso/knop.las");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lasso".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Lasso/json.lasso9");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lasso".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Lasso/json.lasso");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lasso".to_string());
        assert_eq!(actual, expected);
    }
}

mod latte {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Latte/template.latte");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Latte".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Latte/layout.latte");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Latte".to_string());
        assert_eq!(actual, expected);
    }
}

mod lean {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Lean/set.hlean");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lean".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Lean/binary.lean");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lean".to_string());
        assert_eq!(actual, expected);
    }
}

mod less {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Less/screen.less");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Less".to_string());
        assert_eq!(actual, expected);
    }
}

mod lex {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Lex/zend_ini_scanner.l");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lex".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Lex/filenames/Lexer.x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lex".to_string());
        assert_eq!(actual, expected);
    }
}

mod limbo {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Limbo/cat.b");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Limbo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Limbo/lock.b");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Limbo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Limbo/lock.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Limbo".to_string());
        assert_eq!(actual, expected);
    }
}

mod linker_script {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Linker Script/vmlinux.lds");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Linker Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Linker Script/link.ld");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Linker Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Linker Script/inject.x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Linker Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Linker Script/filenames/ld.script");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Linker Script".to_string());
        assert_eq!(actual, expected);
    }
}

mod linux_kernel_module {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Linux Kernel Module/bcm4334x.mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Linux Kernel Module".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Linux Kernel Module/md5.mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Linux Kernel Module".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Linux Kernel Module/mbcache.mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Linux Kernel Module".to_string());
        assert_eq!(actual, expected);
    }
}

mod liquid {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Liquid/layout.liquid");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Liquid".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Liquid/template.liquid");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Liquid".to_string());
        assert_eq!(actual, expected);
    }
}

mod literate_agda {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Literate Agda/NatCat.lagda");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Literate Agda".to_string());
        assert_eq!(actual, expected);
    }
}

mod literate_coffeescript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Literate CoffeeScript/pixi.coffee.md");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Literate CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Literate CoffeeScript/scope.litcoffee");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Literate CoffeeScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod livescript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/LiveScript/hello.ls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LiveScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod logos {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Logos/NoCarrier.x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Logos".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Logos/Tweak.x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Logos".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Logos/example.xm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Logos".to_string());
        assert_eq!(actual, expected);
    }

    #[ignore]
    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Logos/string1.x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Logos".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Logos/NCHax.x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Logos".to_string());
        assert_eq!(actual, expected);
    }
}

mod logtalk {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Logtalk/foo.lgt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Logtalk".to_string());
        assert_eq!(actual, expected);
    }
}

mod lookml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/LookML/example.model.lkml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LookML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/LookML/example.view.lkml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LookML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/LookML/comments.view.lookml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LookML".to_string());
        assert_eq!(actual, expected);
    }
}

mod loomscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/LoomScript/SyntaxExercise.ls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LoomScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/LoomScript/HelloWorld.ls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("LoomScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod lua {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Lua/vidya-file-list-parser.pd_lua");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lua".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Lua/vidya-file-modder.pd_lua");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lua".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Lua/wsapi.fcgi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lua".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Lua/h-counter.pd_lua");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lua".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Lua/treegen.p8");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lua".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Lua/luatexts-0.1.2-1.rockspec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lua".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Lua/filenames/.luacheckrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Lua".to_string());
        assert_eq!(actual, expected);
    }
}

mod m {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/M/ifelse.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/M/Comment.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/M/MDB.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/M/mumtris.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/M/_zewdDemo.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/M/digest.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/M/_zewdAPI.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/M/helloworld.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/M/primes.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/M/arrays.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/M/PXAI.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/M/pcreexamples.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/M/md5.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/M/pcre.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/M/dynamicscoping.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/M/zmwire.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/M/ZDIOUT1.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/M/forloop.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/M/indirectfunctions.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/M/mileage.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/M/WVBRNOT.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/M/url.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/M/postconditional.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/M/base64.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/M/functions.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/M/nesting.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/M/GMRGPNB0.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/M/PRCAAPR.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/M/fibonacci.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M".to_string());
        assert_eq!(actual, expected);
    }
}

mod m4 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/M4/translit2.m4");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M4".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/M4/htmlgen.m4");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M4".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/M4/postscript.m4");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M4".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/M4/fibo.m4");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M4".to_string());
        assert_eq!(actual, expected);
    }
}

mod m4sugar {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/M4Sugar/list.m4");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M4Sugar".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/M4Sugar/ax_ruby_devel.m4");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M4Sugar".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/M4Sugar/filenames/configure.ac");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("M4Sugar".to_string());
        assert_eq!(actual, expected);
    }
}

mod matlab {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/MATLAB/create_ieee_paper_plots.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/MATLAB/Poincare.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/MATLAB/convert_variable.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/MATLAB/test_rk_par.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/MATLAB/double_gyre.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/MATLAB/gpu_RKF45_FILE.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/MATLAB/Lagr.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/MATLAB/FTLE_reg.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/MATLAB/cross_validation.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/MATLAB/Integrate1.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/MATLAB/plant.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/MATLAB/Check_plot.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/MATLAB/lane_change.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/MATLAB/average.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/MATLAB/matlab_function.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/MATLAB/fit_adapt_linear.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/MATLAB/load_bikes.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/MATLAB/Integrate2.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/MATLAB/load_data.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/MATLAB/bicycle_state_space.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/MATLAB/adapting_structural_model.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/MATLAB/make_filter.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/MATLAB/overwrite_settings.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/MATLAB/matlab_script.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/MATLAB/matlab_script2.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/MATLAB/ieee.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/MATLAB/fit_adapt.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/MATLAB/matlab_class.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/MATLAB/par_text_to_struct.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_29() {
        let path = Path::new(&"./samples/MATLAB/test_system_state_space.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_30() {
        let path = Path::new(&"./samples/MATLAB/varargin_to_structure.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_31() {
        let path = Path::new(&"./samples/MATLAB/normalize.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_32() {
        let path = Path::new(&"./samples/MATLAB/write_gains.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_33() {
        let path = Path::new(&"./samples/MATLAB/distance.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_34() {
        let path = Path::new(&"./samples/MATLAB/Lagrangian_points.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_35() {
        let path = Path::new(&"./samples/MATLAB/RK4.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_36() {
        let path = Path::new(&"./samples/MATLAB/FTLEH.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_37() {
        let path = Path::new(&"./samples/MATLAB/example.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_38() {
        let path = Path::new(&"./samples/MATLAB/Traj.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MATLAB".to_string());
        assert_eq!(actual, expected);
    }
}

mod maxscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/MAXScript/macro-1.mcr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MAXScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/MAXScript/rolloutCreator.ms");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MAXScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/MAXScript/svg-renderer.ms");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MAXScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/MAXScript/volume-calc.ms");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MAXScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/MAXScript/macro-2.mcr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MAXScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod mlir {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/MLIR/sample.mlir");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MLIR".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/MLIR/executor_to_control_dialect.mlir");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MLIR".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/MLIR/const-fold.mlir");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MLIR".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/MLIR/core-ops.mlir");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MLIR".to_string());
        assert_eq!(actual, expected);
    }
}

mod mql4 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/MQL4/indicator-sample.mq4");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MQL4".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/MQL4/script-sample.mq4");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MQL4".to_string());
        assert_eq!(actual, expected);
    }

    #[ignore]
    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/MQL4/header-sample.mqh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MQL4".to_string());
        assert_eq!(actual, expected);
    }
}

mod mql5 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/MQL5/indicator-sample.mq5");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MQL5".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/MQL5/Regex.mqh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MQL5".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/MQL5/script-sample.mq5");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MQL5".to_string());
        assert_eq!(actual, expected);
    }
}

mod mtml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/MTML/categories_to_columns.mtml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MTML".to_string());
        assert_eq!(actual, expected);
    }
}

mod muf {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/MUF/39.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MUF".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/MUF/cmd-say.muf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MUF".to_string());
        assert_eq!(actual, expected);
    }
}

mod macaulay2 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Macaulay2/FGLM.m2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Macaulay2".to_string());
        assert_eq!(actual, expected);
    }
}

mod makefile {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Makefile/makefile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Makefile/foo.o.d");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Makefile/file-icons.make");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Makefile/filenames/Makefile.boot");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Makefile/filenames/makefile.sco");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Makefile/filenames/Kbuild");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Makefile/filenames/Makefile.wat");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Makefile/filenames/Makefile.frag");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Makefile/filenames/Makefile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Makefile/filenames/mkfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Makefile/filenames/BSDmakefile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Makefile/filenames/Makefile.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Makefile".to_string());
        assert_eq!(actual, expected);
    }
}

mod markdown {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Markdown/symlink.md");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Markdown/ronn-format.7.ronn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Markdown/ronn.1.ronn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Markdown/minimal.md");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Markdown/bunyan.1.ronn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Markdown/sway.5.scd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Markdown/tender.md");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Markdown/csharp6.workbook");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Markdown/README.mdown");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Markdown/sample.mdx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Markdown/filenames/contents.lr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Markdown".to_string());
        assert_eq!(actual, expected);
    }
}

mod marko {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Marko/counter.marko");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Marko".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Marko/hello.marko");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Marko".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Marko/rgb-sliders.marko");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Marko".to_string());
        assert_eq!(actual, expected);
    }
}

mod mask {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Mask/view.mask");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mask".to_string());
        assert_eq!(actual, expected);
    }
}

mod mathematica {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Mathematica/Problem12.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Mathematica/Predicates.wl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Mathematica/PacletInfo.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Mathematica/TestArithmetic.mt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Mathematica/MiscCalculations.nb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Mathematica/MiscCalculations2.nb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Mathematica/Predicates.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Mathematica/HeyexImport.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Mathematica/UnitTest.wlt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Mathematica/TestSuite.mt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Mathematica/TestString.mt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Mathematica/Init.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mathematica".to_string());
        assert_eq!(actual, expected);
    }
}

mod maven_pom {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Maven POM/filenames/pom.xml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Maven POM".to_string());
        assert_eq!(actual, expected);
    }
}

mod max {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Max/Hello.maxpat");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Max".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Max/Hello.mxt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Max".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Max/Hello.maxhelp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Max".to_string());
        assert_eq!(actual, expected);
    }
}

mod mercury {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Mercury/rot13_concise.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mercury".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Mercury/switch_detection_bug.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mercury".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Mercury/code_info.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mercury".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Mercury/hello.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mercury".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Mercury/store.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mercury".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Mercury/options.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mercury".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Mercury/rot13_ralph.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mercury".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Mercury/polymorphism.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mercury".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Mercury/rot13_verbose.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mercury".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Mercury/expr.moo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mercury".to_string());
        assert_eq!(actual, expected);
    }
}

mod meson {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Meson/filenames/meson.build");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Meson".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Meson/filenames/meson_options.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Meson".to_string());
        assert_eq!(actual, expected);
    }
}

mod metal {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Metal/ITMVisualisationEngine.metal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Metal".to_string());
        assert_eq!(actual, expected);
    }
}

mod microsoft_developer_studio_project {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Microsoft Developer Studio Project/freeglut.dsp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Microsoft Developer Studio Project".to_string());
        assert_eq!(actual, expected);
    }
}

mod microsoft_visual_studio_solution {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Microsoft Visual Studio Solution/Radiant.sln");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Microsoft Visual Studio Solution".to_string());
        assert_eq!(actual, expected);
    }
}

mod modelica {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Modelica/package4.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Modelica/Pendulum.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Modelica/package.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Modelica/System.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Modelica/RLC.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Modelica/SecondOrderSystem.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Modelica/NestedPackages.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Modelica/modelica.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Modelica/package3.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Modelica/package2.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Modelica/Translational.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Modelica/NewtonCooling.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modelica".to_string());
        assert_eq!(actual, expected);
    }
}

mod modula_2 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Modula-2/HuffChan.mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modula-2".to_string());
        assert_eq!(actual, expected);
    }
}

mod modula_3 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Modula-3/RdClass.i3");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modula-3".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Modula-3/DiGraph.mg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modula-3".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Modula-3/DiGraph.ig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modula-3".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Modula-3/Rd.m3");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modula-3".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Modula-3/Rd.i3");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Modula-3".to_string());
        assert_eq!(actual, expected);
    }
}

mod module_management_system {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Module Management System/descrip.mms");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Module Management System".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Module Management System/simh_descrip.mms");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Module Management System".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Module Management System/spline_descrip.mms");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Module Management System".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Module Management System/xv_makefile.mms");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Module Management System".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Module Management System/openvms.mmk");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Module Management System".to_string());
        assert_eq!(actual, expected);
    }
}

mod monkey {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Monkey/gui.monkey2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Monkey".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Monkey/encodeToPng.monkey2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Monkey".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Monkey/example.monkey");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Monkey".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Monkey/sorting.monkey2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Monkey".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Monkey/example.monkey2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Monkey".to_string());
        assert_eq!(actual, expected);
    }
}

mod moocode {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Moocode/moocode_toolkit.moo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Moocode".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Moocode/hello.moo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Moocode".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Moocode/toy.moo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Moocode".to_string());
        assert_eq!(actual, expected);
    }
}

mod moonscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/MoonScript/transform.moon");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("MoonScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod motoko {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Motoko/erc20.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Motoko".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Motoko/queue.mo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Motoko".to_string());
        assert_eq!(actual, expected);
    }
}

mod motorola_68k_assembly {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Motorola 68K Assembly/system.s");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Motorola 68K Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Motorola 68K Assembly/lz4.X68");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Motorola 68K Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Motorola 68K Assembly/cpu.s");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Motorola 68K Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Motorola 68K Assembly/bls_routines.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Motorola 68K Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Motorola 68K Assembly/iff_ilbm.i");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Motorola 68K Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Motorola 68K Assembly/rom_testbench.asm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Motorola 68K Assembly".to_string());
        assert_eq!(actual, expected);
    }
}

mod muse {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Muse/usage.muse");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Muse".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Muse/manual.muse");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Muse".to_string());
        assert_eq!(actual, expected);
    }
}

mod mustache {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Mustache/pupilinfoblock.mustache");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mustache".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Mustache/deleteuser.mustache");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mustache".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Mustache/csvrow.mustache");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mustache".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Mustache/showallusers.mustache");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Mustache".to_string());
        assert_eq!(actual, expected);
    }
}

mod nasl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NASL/cmsimple_search_xss.nasl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NASL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/NASL/hacker_defender.nasl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NASL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/NASL/smtp_func.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NASL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/NASL/cmsimple_guestbook_xss.nasl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NASL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/NASL/data_mail_xss.nasl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NASL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/NASL/tftp_func.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NASL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/NASL/packeteer_web_login.nasl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NASL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/NASL/nmap.nasl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NASL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/NASL/bugbear_b.nasl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NASL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/NASL/cubecart_xss.nasl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NASL".to_string());
        assert_eq!(actual, expected);
    }
}

mod ncl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NCL/viewport_4.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/NCL/traj_3.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/NCL/tsdiagram_1.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/NCL/mask_12.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/NCL/weather_sym_6.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/NCL/WRF_static_2.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/NCL/unique_9.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/NCL/mcsst_1.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/NCL/xy_29.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/NCL/topo_9.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/NCL/PrnOscPat_driver.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/NCL/gsn_csm_xy2_time_series_inputs.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/NCL/WRF_track_1.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/NCL/hdf4sds_7.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/NCL/primero.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/NCL/cru_8.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NCL".to_string());
        assert_eq!(actual, expected);
    }
}

mod neon {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NEON/example.neon");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NEON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/NEON/config.neon");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NEON".to_string());
        assert_eq!(actual, expected);
    }
}

mod nl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NL/balassign0.nl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/NL/assign0.nl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NL".to_string());
        assert_eq!(actual, expected);
    }
}

mod npm_config {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NPM Config/filenames/.npmrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NPM Config".to_string());
        assert_eq!(actual, expected);
    }
}

mod nsis {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NSIS/bigtest.nsi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NSIS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/NSIS/x64.nsh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NSIS".to_string());
        assert_eq!(actual, expected);
    }
}

mod nwscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NWScript/crawler_death.nss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NWScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/NWScript/e3pc_spidersnest.nss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NWScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/NWScript/illusion_spawn.nss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NWScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/NWScript/act_disarmkobold.nss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NWScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/NWScript/act_unlockkobold.nss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NWScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/NWScript/e2pc_plaguestack.nss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NWScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod nearley {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Nearley/nearley-language-bootstrapped.ne");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nearley".to_string());
        assert_eq!(actual, expected);
    }
}

mod nemerle {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Nemerle/hello.n");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nemerle".to_string());
        assert_eq!(actual, expected);
    }
}

mod netlinx {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NetLinx/volume-array.axs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NetLinx".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/NetLinx/projector.axi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NetLinx".to_string());
        assert_eq!(actual, expected);
    }
}

mod netlinx_plus_erb {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NetLinx+ERB/sample.axi.erb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NetLinx+ERB".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/NetLinx+ERB/sample.axs.erb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NetLinx+ERB".to_string());
        assert_eq!(actual, expected);
    }
}

mod netlogo {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NetLogo/Life.nlogo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NetLogo".to_string());
        assert_eq!(actual, expected);
    }
}

mod newlisp {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/NewLisp/log-to-database.lisp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NewLisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/NewLisp/queens.nl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NewLisp".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/NewLisp/irc.lsp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("NewLisp".to_string());
        assert_eq!(actual, expected);
    }
}

mod nextflow {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Nextflow/callings.nf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nextflow".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Nextflow/rnaseq.nf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nextflow".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Nextflow/blast.nf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nextflow".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Nextflow/filenames/nextflow.config");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nextflow".to_string());
        assert_eq!(actual, expected);
    }
}

mod nginx {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Nginx/sample.nginx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nginx".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Nginx/example.com.vhost");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nginx".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Nginx/filenames/nginx.conf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nginx".to_string());
        assert_eq!(actual, expected);
    }
}

mod nim {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Nim/nimfix.nim.cfg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nim".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Nim/zip.nimble");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nim".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Nim/config.nims");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nim".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Nim/main.nim");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nim".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Nim/foo.nim");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nim".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Nim/filenames/nim.cfg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nim".to_string());
        assert_eq!(actual, expected);
    }
}

mod nit {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Nit/calculator.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Nit/curl_http.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Nit/html_page.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Nit/callback_monkey.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Nit/fibonacci.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Nit/tmpl_composer.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Nit/circular_list.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Nit/socket_client.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Nit/drop_privileges.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Nit/clock.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Nit/procedural_array.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Nit/curl_mail.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Nit/draw_operation.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Nit/file.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Nit/clock_more.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Nit/print_arguments.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Nit/hello_world.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Nit/websocket_server.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Nit/extern_methods.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Nit/meetup.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Nit/callback_chimpanze.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Nit/int_stack.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Nit/opengles2_hello_triangle.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/Nit/socket_server.nit");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nit".to_string());
        assert_eq!(actual, expected);
    }
}

mod nix {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Nix/nginx.nix");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nix".to_string());
        assert_eq!(actual, expected);
    }
}

mod nu {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Nu/RandomApp.nu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nu".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Nu/nu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nu".to_string());
        assert_eq!(actual, expected);
    }
}

mod nunjucks {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Nunjucks/norris.njk");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Nunjucks".to_string());
        assert_eq!(actual, expected);
    }
}

mod ocaml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/OCaml/uutf.ml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OCaml".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/OCaml/cmdliner.ml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OCaml".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/OCaml/date.ml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OCaml".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/OCaml/reload.ml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OCaml".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/OCaml/example.eliom");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OCaml".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/OCaml/mirage.ml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OCaml".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/OCaml/sigset.ml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OCaml".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/OCaml/common.ml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OCaml".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/OCaml/Foo.ml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OCaml".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/OCaml/map.ml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OCaml".to_string());
        assert_eq!(actual, expected);
    }
}

mod object_data_instance_notation {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Object Data Instance Notation/openehr_ehr_1.0.3.bmm.odin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Object Data Instance Notation".to_string());
        assert_eq!(actual, expected);
    }
}

mod objectscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ObjectScript/Sample.Person.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ObjectScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod objective_c {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Objective-C/FooAppDelegate.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Objective-C/StyleViewController.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Objective-C/ASIHTTPRequest.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Objective-C/MainMenuViewController.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Objective-C/SBJsonParser.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Objective-C/hello.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Objective-C/ASIHTTPRequest.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Objective-C/FooAppDelegate.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Objective-C/Foo.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Objective-C/PlaygroundViewController.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Objective-C/JSONKit.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Objective-C/StyleViewController.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Objective-C/TUITableView.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Objective-C/MainMenuViewController.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Objective-C/Siesta.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Objective-C/PlaygroundViewController.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Objective-C/SBJsonParser.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Objective-C/cocoa_monitor.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Objective-C/JSONKit.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Objective-C/TUITableView.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Objective-C/Foo.h");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Objective-C/gen-cocoa-linguist-thrift.m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C".to_string());
        assert_eq!(actual, expected);
    }
}

mod objective_c_plus__plus_ {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Objective-C++/objsql.mm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C++".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Objective-C++/EventHandlerMac.mm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-C++".to_string());
        assert_eq!(actual, expected);
    }
}

mod objective_j {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Objective-J/LightsOff.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-J".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Objective-J/iTunesLayout.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-J".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Objective-J/AppController.j");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Objective-J".to_string());
        assert_eq!(actual, expected);
    }
}

mod odin {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Odin/sample.odin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Odin".to_string());
        assert_eq!(actual, expected);
    }
}

mod omgrofl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Omgrofl/hello.omgrofl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Omgrofl".to_string());
        assert_eq!(actual, expected);
    }
}

mod opa {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Opa/hello_syntax2.opa");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Opa".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Opa/hello_syntax1.opa");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Opa".to_string());
        assert_eq!(actual, expected);
    }
}

mod opal {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Opal/DeepakChopra.opal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Opal".to_string());
        assert_eq!(actual, expected);
    }
}

mod open_policy_agent {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Open Policy Agent/kubernetes_admission.rego");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Open Policy Agent".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Open Policy Agent/ssh.rego");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Open Policy Agent".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Open Policy Agent/httpapi.rego");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Open Policy Agent".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Open Policy Agent/kafka.rego");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Open Policy Agent".to_string());
        assert_eq!(actual, expected);
    }
}

mod opencl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/OpenCL/sample.cl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/OpenCL/fft.cl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenCL".to_string());
        assert_eq!(actual, expected);
    }
}

mod openedge_abl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/OpenEdge ABL/openedge.p");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenEdge ABL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/OpenEdge ABL/SendEmailAlgorithm.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenEdge ABL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/OpenEdge ABL/SocketReader.p");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenEdge ABL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/OpenEdge ABL/test-win.w");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenEdge ABL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/OpenEdge ABL/Email.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenEdge ABL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/OpenEdge ABL/Util.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenEdge ABL".to_string());
        assert_eq!(actual, expected);
    }
}

mod openqasm {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/OpenQASM/half_adder.qasm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenQASM".to_string());
        assert_eq!(actual, expected);
    }
}

mod openrc_runscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/OpenRC runscript/acpid");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenRC runscript".to_string());
        assert_eq!(actual, expected);
    }
}

mod openscad {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/OpenSCAD/not_simple.scad");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenSCAD".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/OpenSCAD/simple.scad");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenSCAD".to_string());
        assert_eq!(actual, expected);
    }
}

mod openstep_property_list {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/OpenStep Property List/TypewriterInfo.plist");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenStep Property List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/OpenStep Property List/weathericons-regular.glyphs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("OpenStep Property List".to_string());
        assert_eq!(actual, expected);
    }
}

mod org {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Org/org.org");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Org".to_string());
        assert_eq!(actual, expected);
    }
}

mod ox {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Ox/ParallelObjective.ox");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ox".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Ox/IJCEmet2009.oxh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ox".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Ox/particle.oxo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ox".to_string());
        assert_eq!(actual, expected);
    }
}

mod oxygene {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Oxygene/Loops.oxygene");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Oxygene".to_string());
        assert_eq!(actual, expected);
    }
}

mod oz {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Oz/example.oz");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Oz".to_string());
        assert_eq!(actual, expected);
    }
}

mod p4 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/P4/mirror_acl.p4");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("P4".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/P4/l2.p4");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("P4".to_string());
        assert_eq!(actual, expected);
    }
}

mod peg_dot_js {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PEG.js/rfc5988.pegjs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PEG.js".to_string());
        assert_eq!(actual, expected);
    }
}

mod php {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PHP/mail.phps");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/PHP/Application.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/PHP/php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/PHP/Model.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/PHP/ThriftGenerated.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/PHP/root.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/PHP/file_display.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/PHP/Form.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/PHP/ProtobufGenerated.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/PHP/prefix.fcgi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/PHP/drupal.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/PHP/Controller.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/PHP/php-script");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/PHP/Client.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/PHP/php2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/PHP/exception.zep.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/PHP/filenames/.php_cs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/PHP/filenames/.php");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/PHP/filenames/.php_cs.dist");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PHP".to_string());
        assert_eq!(actual, expected);
    }
}

mod plsql {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PLSQL/packagebody.pkb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/PLSQL/plsqlguide.pck");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/PLSQL/prime#.plsql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/PLSQL/videodb.ddl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/PLSQL/packageheader.pks");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/PLSQL/myobject.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/PLSQL/who_called_me.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[ignore]
    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/PLSQL/print_bool.prc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLSQL".to_string());
        assert_eq!(actual, expected);
    }
}

mod plpgsql {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PLpgSQL/plpgsql_lint-9.1.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLpgSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/PLpgSQL/plpgsql_lint-9.2.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLpgSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/PLpgSQL/procedures.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLpgSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/PLpgSQL/useraccount.pgsql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLpgSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/PLpgSQL/plpgsql_lint-8.4.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLpgSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/PLpgSQL/plpgsql_lint-9.3.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLpgSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/PLpgSQL/plpgsql_lint-9.0.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PLpgSQL".to_string());
        assert_eq!(actual, expected);
    }
}

mod pov_ray_sdl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/POV-Ray SDL/terrain.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/POV-Ray SDL/cloth.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/POV-Ray SDL/table.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/POV-Ray SDL/gamma_showcase.pov");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/POV-Ray SDL/table_cloth.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/POV-Ray SDL/table_stuff.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/POV-Ray SDL/building.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/POV-Ray SDL/water.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/POV-Ray SDL/chair.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/POV-Ray SDL/balcony.pov");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/POV-Ray SDL/bglass.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/POV-Ray SDL/sky.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("POV-Ray SDL".to_string());
        assert_eq!(actual, expected);
    }
}

mod pan {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pan/databases.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Pan/unit.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Pan/simple.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Pan/link.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Pan/libvirt.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Pan/cluster-A.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Pan/functions.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Pan/ceph-raid.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Pan/osd-fetch.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Pan/pakiti.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Pan/infernalis.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Pan/test.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Pan/resources.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Pan/nodes_properties.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Pan/types.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Pan/mysql.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Pan/purge_fqan_accounts.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Pan/onevm.pan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pan".to_string());
        assert_eq!(actual, expected);
    }
}

mod papyrus {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Papyrus/CAMTEST_OverShoulderME.psc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Papyrus".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Papyrus/vSCM_MetaQuestScript.psc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Papyrus".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Papyrus/vMFX_FXPlugin.psc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Papyrus".to_string());
        assert_eq!(actual, expected);
    }
}

mod parrot_assembly {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Parrot Assembly/hello.pasm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Parrot Assembly".to_string());
        assert_eq!(actual, expected);
    }
}

mod parrot_internal_representation {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Parrot Internal Representation/hello.pir");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Parrot Internal Representation".to_string());
        assert_eq!(actual, expected);
    }
}

mod pascal {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pascal/lazcomunit.pas");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Pascal/program.dpr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Pascal/image_url.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Pascal/libc.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Pascal/GraphicConfiguration.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Pascal/tw27294.pp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Pascal/lazcomlib_1_0_tlb.pas");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Pascal/cwindirs.pp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Pascal/read-a-configuration-file.pascal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Pascal/custforms.pp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Pascal/bulls-and-cows.pascal");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Pascal/large.pp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Pascal/vmops_impl.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pascal".to_string());
        assert_eq!(actual, expected);
    }
}

mod pawn {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pawn/y_testing.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pawn".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Pawn/Check.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pawn".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Pawn/fixed.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pawn".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Pawn/mfile.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pawn".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Pawn/fixes.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pawn".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Pawn/timertest.pwn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pawn".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Pawn/foo.sma");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pawn".to_string());
        assert_eq!(actual, expected);
    }
}

mod pep8 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pep8/div.pep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pep8".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Pep8/linked.pep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pep8".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Pep8/stri_buf.pep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pep8".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Pep8/msq.pep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pep8".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Pep8/qsort.pep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pep8".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Pep8/stristack.pep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pep8".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Pep8/flag.pep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pep8".to_string());
        assert_eq!(actual, expected);
    }
}

mod perl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Perl/Ack.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Perl/use5.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Perl/getchar.al");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Perl/exception_handler.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Perl/oo3.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Perl/oo1.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Perl/example.cgi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Perl/strict.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Perl/script.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Perl/Any.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Perl/perl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Perl/Request.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Perl/fib.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Perl/test-perl.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Perl/Response.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Perl/oo2.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Perl/perl-test.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Perl/feedgnuplot");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Perl/index.fcgi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[ignore]
    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Perl/test-perl2.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Perl/filenames/Makefile.PL");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Perl/filenames/ack");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Perl/filenames/cpanfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/Perl/filenames/Rexfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Perl".to_string());
        assert_eq!(actual, expected);
    }
}

mod pic {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pic/dextroamphetamine.chem");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Pic/ritalin.chem");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Pic/graph.pic");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pic".to_string());
        assert_eq!(actual, expected);
    }
}

mod pickle {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pickle/save.pkl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pickle".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Pickle/data.pkl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pickle".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Pickle/neural-network-ce-l2reg-784-10-30.pkl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pickle".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Pickle/random.pkl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pickle".to_string());
        assert_eq!(actual, expected);
    }
}

mod picolisp {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PicoLisp/simul.l");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PicoLisp".to_string());
        assert_eq!(actual, expected);
    }
}

mod piglatin {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PigLatin/example.pig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PigLatin".to_string());
        assert_eq!(actual, expected);
    }
}

mod pike {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pike/shebang.pike");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pike".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Pike/Error.pmod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pike".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Pike/FakeFile.pike");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pike".to_string());
        assert_eq!(actual, expected);
    }
}

mod plantuml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PlantUML/casUtilisation.puml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PlantUML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/PlantUML/overview.puml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PlantUML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/PlantUML/PublicallyAccessibleReDirect.puml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PlantUML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/PlantUML/sequence-ptasks-workers_success.puml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PlantUML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/PlantUML/ProtectedMeta.iuml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PlantUML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/PlantUML/associations.iuml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PlantUML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/PlantUML/common.iuml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PlantUML".to_string());
        assert_eq!(actual, expected);
    }
}

mod pod {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pod/Sample.pod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pod".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Pod/contents.pod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pod".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Pod/Cookbook.pod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pod".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Pod/PSGI.pod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pod".to_string());
        assert_eq!(actual, expected);
    }
}

mod pod_6 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pod 6/S15-unicode.pod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pod 6".to_string());
        assert_eq!(actual, expected);
    }
}

mod pogoscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PogoScript/squashy.pogo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PogoScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod pony {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pony/mandelbrot.pony");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pony".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Pony/hello-world.pony");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pony".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Pony/counter.pony");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pony".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Pony/circle.pony");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pony".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Pony/gups-opt.pony");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pony".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Pony/mixed.pony");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pony".to_string());
        assert_eq!(actual, expected);
    }
}

mod postcss {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PostCSS/sample.postcss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PostCSS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/PostCSS/sample.pcss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PostCSS".to_string());
        assert_eq!(actual, expected);
    }
}

mod postscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PostScript/xlogo.epsi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PostScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/PostScript/lambda.pfa");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PostScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/PostScript/sierpinski.ps");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PostScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod powerbuilder {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PowerBuilder/w_export.srw");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PowerBuilder".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/PowerBuilder/part1.srw");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PowerBuilder".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/PowerBuilder/myproject.pbt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PowerBuilder".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/PowerBuilder/TestPBT.pbt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PowerBuilder".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/PowerBuilder/n_cst_buttonlistbar_gradient.sru");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PowerBuilder".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/PowerBuilder/ginpix7.sra");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PowerBuilder".to_string());
        assert_eq!(actual, expected);
    }
}

mod powershell {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PowerShell/ZLocation.psm1");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PowerShell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/PowerShell/ZLocation.psd1");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PowerShell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/PowerShell/history.ps1");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PowerShell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/PowerShell/pwsh-shebang.ps1");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PowerShell".to_string());
        assert_eq!(actual, expected);
    }
}

mod prisma {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Prisma/schema.prisma");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prisma".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Prisma/mcu-schema.prisma");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prisma".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Prisma/now-example-schema.prisma");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prisma".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Prisma/blog-schema-advanced.prisma");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prisma".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Prisma/blog-minimal-schema.prisma");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prisma".to_string());
        assert_eq!(actual, expected);
    }
}

mod processing {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Processing/hello.pde");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Processing".to_string());
        assert_eq!(actual, expected);
    }
}

mod proguard {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Proguard/proguard-rules2.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Proguard".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Proguard/proguard-rules.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Proguard".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Proguard/proguard_annotations.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Proguard".to_string());
        assert_eq!(actual, expected);
    }
}

mod prolog {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Prolog/ex6.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prolog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Prolog/test-prolog.prolog");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prolog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Prolog/format_spec.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prolog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Prolog/queues.yap");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prolog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Prolog/admin.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prolog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Prolog/turing.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prolog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Prolog/func.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prolog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Prolog/dleak-report");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prolog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Prolog/logic-problem.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Prolog".to_string());
        assert_eq!(actual, expected);
    }
}

mod promela {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Promela/Supervisor.pml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Promela".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Promela/Session.pml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Promela".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Promela/Thread.pml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Promela".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Promela/attacker_4_FINITE.pml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Promela".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Promela/TCP.pml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Promela".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Promela/bare_signals.pml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Promela".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Promela/ex.1.pml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Promela".to_string());
        assert_eq!(actual, expected);
    }
}

mod propeller_spin {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Propeller Spin/4x4 Keypad Reader.spin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Propeller Spin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Propeller Spin/TV.spin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Propeller Spin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Propeller Spin/Inductor.spin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Propeller Spin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Propeller Spin/TV_Terminal.spin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Propeller Spin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Propeller Spin/Graphics.spin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Propeller Spin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Propeller Spin/Keyboard.spin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Propeller Spin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Propeller Spin/Debug_Lcd.spin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Propeller Spin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Propeller Spin/TV_Text.spin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Propeller Spin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Propeller Spin/VGA.spin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Propeller Spin".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Propeller Spin/VocalTract.spin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Propeller Spin".to_string());
        assert_eq!(actual, expected);
    }
}

mod protocol_buffer {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Protocol Buffer/addressbook.proto");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Protocol Buffer".to_string());
        assert_eq!(actual, expected);
    }
}

mod public_key {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Public Key/sunCert.asc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Public Key".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Public Key/id.pub");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Public Key".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Public Key/id_rsa.pub");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Public Key".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Public Key/gpg_key.asc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Public Key".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Public Key/id_dsa.asc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Public Key".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Public Key/sshkey1.asc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Public Key".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Public Key/id_rsa.asc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Public Key".to_string());
        assert_eq!(actual, expected);
    }
}

mod pug {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Pug/hello.pug");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pug".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Pug/hello.jade");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Pug".to_string());
        assert_eq!(actual, expected);
    }
}

mod puppet {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Puppet/stages-example.pp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Puppet".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Puppet/hiera_include.pp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Puppet".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Puppet/expiringhost.pp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Puppet".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Puppet/apacheinit.pp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Puppet".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Puppet/unmanaged-notify-puppet25.pp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Puppet".to_string());
        assert_eq!(actual, expected);
    }
}

mod purebasic {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PureBasic/Memory.pbi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PureBasic".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/PureBasic/Example_Sine.pb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PureBasic".to_string());
        assert_eq!(actual, expected);
    }
}

mod purescript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/PureScript/ReactiveJQueryTest.purs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PureScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/PureScript/Data.Map.purs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PureScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/PureScript/Data.Foreign.purs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PureScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/PureScript/Control.Arrow.purs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("PureScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod python {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Python/python2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Python/python");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Python/action.cgi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Python/python3");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Python/snakemake-mapping.smk");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Python/toolchain.gypi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Python/flask-view.py");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Python/django-models-base.py");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Python/backstage.fcgi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Python/snakemake-calling.smk");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Python/gen-py-linguist-thrift.py");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Python/spec.linux.spec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Python/simpleclient.rpy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Python/AdditiveWave.pyde");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Python/protocol_buffer_pb2.py");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Python/standalone.gypi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Python/MoveEye.pyde");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Python/argparse.pyi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Python/Cinema4DPythonPlugin.pyp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Python/py3.py3");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Python/tornado-httpserver.py");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Python/filenames/DEPS");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Python/filenames/.gclient");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Python".to_string());
        assert_eq!(actual, expected);
    }
}

mod q_sharp_ {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Q#/CustomModAdd.qs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Q#".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Q#/Shor.qs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Q#".to_string());
        assert_eq!(actual, expected);
    }
}

mod qml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/QML/common.qbs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("QML".to_string());
        assert_eq!(actual, expected);
    }
}

mod qmake {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/QMake/functions.pri");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("QMake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/QMake/complex.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("QMake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/QMake/simple.pro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("QMake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/QMake/qmake");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("QMake".to_string());
        assert_eq!(actual, expected);
    }
}

mod qt_script {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Qt Script/toolchain_installscript.qs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Qt Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Qt Script/installscript.qs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Qt Script".to_string());
        assert_eq!(actual, expected);
    }
}

mod quake {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Quake/filenames/m3overrides");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Quake".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Quake/filenames/m3makefile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Quake".to_string());
        assert_eq!(actual, expected);
    }
}

mod r {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/R/df.residual.r");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("R".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/R/R-qgis-extension.rsx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("R".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/R/hello-r.R");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("R".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/R/scholar.Rd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("R".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/R/import.Rd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("R".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/R/git-punchcard");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("R".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/R/import.r");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("R".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/R/filenames/expr-dist");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("R".to_string());
        assert_eq!(actual, expected);
    }
}

mod raml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/RAML/api.raml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RAML".to_string());
        assert_eq!(actual, expected);
    }
}

mod rdoc {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/RDoc/rdoc.rdoc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RDoc".to_string());
        assert_eq!(actual, expected);
    }
}

mod rexx {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/REXX/BatchRemapBrushes.pprx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("REXX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/REXX/ag2xml.rexx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("REXX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/REXX/SkrivShape.rexx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("REXX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/REXX/ShapesInfo.rexx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("REXX".to_string());
        assert_eq!(actual, expected);
    }
}

mod rmarkdown {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/RMarkdown/example.rmd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RMarkdown".to_string());
        assert_eq!(actual, expected);
    }
}

mod rpc {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/RPC/yp.x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RPC".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/RPC/rusers.x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RPC".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/RPC/rpc.x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RPC".to_string());
        assert_eq!(actual, expected);
    }
}

mod rpm_spec {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/RPM Spec/apache.spec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RPM Spec".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/RPM Spec/manos.spec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RPM Spec".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/RPM Spec/erlang-erlydtl.spec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RPM Spec".to_string());
        assert_eq!(actual, expected);
    }
}

mod runoff {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/RUNOFF/longlib.rno");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RUNOFF".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/RUNOFF/contributing.rnh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RUNOFF".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/RUNOFF/mcp_help.rnh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RUNOFF".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/RUNOFF/VMS_ZIP.RNH");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RUNOFF".to_string());
        assert_eq!(actual, expected);
    }
}

mod racket {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Racket/scribble.scrbl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Racket".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Racket/99-bottles-of-beer.scrbl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Racket".to_string());
        assert_eq!(actual, expected);
    }
}

mod ragel {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Ragel/simple_scanner.rl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ragel".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Ragel/ephemeris_parser.rl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ragel".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Ragel/simple_tokenizer.rl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ragel".to_string());
        assert_eq!(actual, expected);
    }
}

mod raku {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Raku/Simple.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Raku/basic-open.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Raku/dynaver.raku");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Raku/listquote-whitespace.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Raku/SIL.rakumod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Raku/ANSIColor.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Raku/List.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Raku/hash.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Raku/01-parse.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Raku/test.p6");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Raku/calendar.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Raku/Bailador.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Raku/Win32.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Raku/Exception.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Raku/ContainsUnicode.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Raku/01-dash-uppercase-i.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Raku/advent2009-day16.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Raku/for.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Raku/htmlify.pl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Raku/Model.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Raku/man-or-boy.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Raku/A.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Raku/grammar-test.p6");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/Raku/RoleQ.pm6");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Raku".to_string());
        assert_eq!(actual, expected);
    }
}

mod rascal {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Rascal/Rascal.rsc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Rascal/Compile.rsc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Rascal/Analyze.rsc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rascal".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Rascal/Syntax.rsc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rascal".to_string());
        assert_eq!(actual, expected);
    }
}

mod rescript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ReScript/RedBlackTree.res");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ReScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod readline_config {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Readline Config/filenames/.inputrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Readline Config".to_string());
        assert_eq!(actual, expected);
    }
}

mod reason {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Reason/Layout.re");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Reason".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Reason/Machine.re");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Reason".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Reason/SourceSpec.re");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Reason".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Reason/JSX.re");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Reason".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Reason/Syntax.re");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Reason".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Reason/SuperMerlin.re");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Reason".to_string());
        assert_eq!(actual, expected);
    }
}

mod rebol {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Rebol/hello-world.reb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rebol".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Rebol/booters.r");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rebol".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Rebol/GCP-datatypes.r");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rebol".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Rebol/hello-world.r2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rebol".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Rebol/hello-world.r3");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rebol".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Rebol/hello-world.rebol");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rebol".to_string());
        assert_eq!(actual, expected);
    }
}

mod record_jar {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Record Jar/filenames/language-subtag-registry.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Record Jar".to_string());
        assert_eq!(actual, expected);
    }
}

mod red {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Red/example.reds");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Red".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Red/example.red");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Red".to_string());
        assert_eq!(actual, expected);
    }
}

mod redirect_rules {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Redirect Rules/filenames/_redirects");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Redirect Rules".to_string());
        assert_eq!(actual, expected);
    }
}

mod regular_expression {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Regular Expression/modeline-emacs.regexp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Regular Expression".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Regular Expression/url.regex");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Regular Expression".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Regular Expression/ordinal.regex");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Regular Expression".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Regular Expression/modeline-vim.regexp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Regular Expression".to_string());
        assert_eq!(actual, expected);
    }
}

mod ren_quote_py {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Ren'Py/example.rpy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ren'Py".to_string());
        assert_eq!(actual, expected);
    }
}

mod renderscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/RenderScript/scenegraph_objects.rsh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RenderScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/RenderScript/convolve3x3.rs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RenderScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod rich_text_format {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Rich Text Format/LICENSE.rtf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rich Text Format".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Rich Text Format/DesktopTemplateLicense.rtf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rich Text Format".to_string());
        assert_eq!(actual, expected);
    }
}

mod ring {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Ring/weighthistory.ring");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ring".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Ring/natural.ring");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ring".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Ring/hello.ring");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ring".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Ring/weblib.ring");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ring".to_string());
        assert_eq!(actual, expected);
    }
}

mod riot {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Riot/todo.riot");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Riot".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Riot/live-filtering.riot");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Riot".to_string());
        assert_eq!(actual, expected);
    }
}

mod robotframework {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/RobotFramework/data_driven.robot");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RobotFramework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/RobotFramework/keyword_driven.robot");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RobotFramework".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/RobotFramework/gherkin.robot");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("RobotFramework".to_string());
        assert_eq!(actual, expected);
    }
}

mod roff {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Roff/printf.3in");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Roff/refs.rno");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Roff/vt.3x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Roff/create_view.l");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Roff/Tcl.n");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Roff/an-ext.tmac");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Roff/he.mdoc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Roff/fsinterface.ms");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Roff/qsort.3qt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Roff/trekmanual.nr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Roff/foo.3pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Roff/dsw.1x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Roff/man.1m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Roff/crude-hack.man");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Roff/roff.1in");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Roff/foo.3p");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Roff/switch.3m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff".to_string());
        assert_eq!(actual, expected);
    }
}

mod roff_manpage {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Roff Manpage/gather_profile_stats.man");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Roff Manpage/tan.3m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Roff Manpage/sched_get_priority_min.3x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Roff Manpage/sensor_attach.mdoc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Roff Manpage/tls_config_ocsp_require_stapling.3in");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Roff Manpage/sigwait.3qt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Roff Manpage/zip_file_add.mdoc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Roff Manpage/pgrep.3p");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Roff Manpage/uname.1m");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Roff Manpage/lyxclient.1in");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Roff Manpage/URI.3pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Roff Manpage/zforce.1x");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Roff Manpage".to_string());
        assert_eq!(actual, expected);
    }
}

mod ruby {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Ruby/index.json.jbuilder");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Ruby/address.pdf.prawn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Ruby/jekyll.rb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Ruby/grit.rb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Ruby/shoes-swt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Ruby/gen-rb-linguist-thrift.rb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Ruby/sinatra.rb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Ruby/racc.rb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Ruby/ruby3");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Ruby/macruby");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Ruby/formula.rb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Ruby/jenkinsci.pluginspec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Ruby/any.spec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Ruby/ruby2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Ruby/rexpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Ruby/rabl.rabl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Ruby/script.rake");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Ruby/inflector.rb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Ruby/gem_loader.rbi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Ruby/resque.rb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Ruby/ruby");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Ruby/mdata_server.fcgi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Ruby/foo.rb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/Ruby/filenames/.pryrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/Ruby/filenames/.irbrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/Ruby/filenames/Podfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/Ruby/filenames/Dangerfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/Ruby/filenames/Rakefile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/Ruby/filenames/Snapfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_29() {
        let path = Path::new(&"./samples/Ruby/filenames/.simplecov");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_30() {
        let path = Path::new(&"./samples/Ruby/filenames/Deliverfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_31() {
        let path = Path::new(&"./samples/Ruby/filenames/Fastfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_32() {
        let path = Path::new(&"./samples/Ruby/filenames/Brewfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_33() {
        let path = Path::new(&"./samples/Ruby/filenames/Appraisals");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_34() {
        let path = Path::new(&"./samples/Ruby/filenames/Capfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Ruby".to_string());
        assert_eq!(actual, expected);
    }
}

mod rust {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Rust/main.rs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rust".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Rust/task.rs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rust".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Rust/hashmap.rs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Rust".to_string());
        assert_eq!(actual, expected);
    }
}

mod sas {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SAS/proc.sas");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SAS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SAS/data.sas");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SAS".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SAS/detect_phi.sas");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SAS".to_string());
        assert_eq!(actual, expected);
    }
}

mod scss {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SCSS/screen.scss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SCSS".to_string());
        assert_eq!(actual, expected);
    }
}

mod selinux_policy {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SELinux Policy/fsck.te");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SELinux Policy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SELinux Policy/filenames/genfs_contexts");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SELinux Policy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SELinux Policy/filenames/security_classes");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SELinux Policy".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/SELinux Policy/filenames/initial_sids");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SELinux Policy".to_string());
        assert_eq!(actual, expected);
    }
}

mod smt {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SMT/shufflevector.smt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SMT".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SMT/bignum_lia1.smt2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SMT".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SMT/queen10-1.smt2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SMT".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/SMT/list4.smt2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SMT".to_string());
        assert_eq!(actual, expected);
    }
}

mod sparql {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SPARQL/foaf.sparql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SPARQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SPARQL/string-matching.sparql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SPARQL".to_string());
        assert_eq!(actual, expected);
    }
}

mod sqf {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SQF/fn_remoteExecFnc.sqf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQF".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SQF/macros.hqf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQF".to_string());
        assert_eq!(actual, expected);
    }
}

mod sql {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SQL/mysql-sakila-schema.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SQL/AvailableInSearchSel.prc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[ignore]
    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SQL/hostcache_set_state.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/SQL/sqlite-sakila-schema.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/SQL/dual.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/SQL/videodb.ddl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/SQL/create_stuff.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/SQL/videodb.cql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/SQL/filial.tab");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/SQL/suspendedtoday.viw");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/SQL/object-update.udf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/SQL/db.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/SQL/drop_stuff.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/SQL/zipcodes.uk.mysql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQL".to_string());
        assert_eq!(actual, expected);
    }
}

mod sqlpl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SQLPL/comm_amount.db2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQLPL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SQLPL/drop_table.db2");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQLPL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SQLPL/sleep.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQLPL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/SQLPL/check_reorg.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQLPL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/SQLPL/trigger.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQLPL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/SQLPL/runstats.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SQLPL".to_string());
        assert_eq!(actual, expected);
    }
}

mod srecode_template {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SRecode Template/linguist.srt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SRecode Template".to_string());
        assert_eq!(actual, expected);
    }
}

mod ssh_config {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SSH Config/filenames/sshd_config");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SSH Config".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SSH Config/filenames/sshconfig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SSH Config".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SSH Config/filenames/ssh-config");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SSH Config".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/SSH Config/filenames/sshd-config");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SSH Config".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/SSH Config/filenames/sshconfig.snip");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SSH Config".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/SSH Config/filenames/ssh_config");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SSH Config".to_string());
        assert_eq!(actual, expected);
    }
}

mod ston {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/STON/Dictionary.ston");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("STON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/STON/ZNResponse.ston");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("STON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/STON/Array.ston");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("STON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/STON/TestDomainObject.ston");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("STON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/STON/properties.ston");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("STON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/STON/methodProperties.ston");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("STON".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/STON/Rectangle.ston");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("STON".to_string());
        assert_eq!(actual, expected);
    }
}

mod swig {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SWIG/CGAL_AABB_tree.i");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SWIG".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SWIG/dictionary.i");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SWIG".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SWIG/gauss.i");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SWIG".to_string());
        assert_eq!(actual, expected);
    }
}

mod sage {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Sage/polinomios.sagews");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sage".to_string());
        assert_eq!(actual, expected);
    }
}

mod saltstack {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SaltStack/top.sls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SaltStack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SaltStack/openoffice.sls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SaltStack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SaltStack/gpg4win-light.sls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SaltStack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/SaltStack/truecrypt.sls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SaltStack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/SaltStack/eval.sls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SaltStack".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/SaltStack/gimp.sls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SaltStack".to_string());
        assert_eq!(actual, expected);
    }
}

mod sass {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Sass/screen.sass");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sass".to_string());
        assert_eq!(actual, expected);
    }
}

mod scala {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Scala/car-ride.kojo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scala".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Scala/scala");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scala".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Scala/turtle-controller.kojo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scala".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Scala/99-bottles-of-beer");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scala".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Scala/fib-tree.kojo");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scala".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Scala/node11.sc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scala".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Scala/build.sbt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scala".to_string());
        assert_eq!(actual, expected);
    }
}

mod scaml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Scaml/hello.scaml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scaml".to_string());
        assert_eq!(actual, expected);
    }
}

mod scheme {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Scheme/basic.sld");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scheme".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Scheme/sboyer.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scheme".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Scheme/lambdastar.sls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scheme".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Scheme/asteroids.sps");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scheme".to_string());
        assert_eq!(actual, expected);
    }
}

mod scilab {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Scilab/scilab_test.tst");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scilab".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Scilab/scilab_script.sce");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scilab".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Scilab/scilab_function.sci");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Scilab".to_string());
        assert_eq!(actual, expected);
    }
}

mod shaderlab {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ShaderLab/DepthOfField.shader");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ShaderLab".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/ShaderLab/Fog.shader");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ShaderLab".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/ShaderLab/Uber.shader");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ShaderLab".to_string());
        assert_eq!(actual, expected);
    }
}

mod shell {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Shell/script.sh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Shell/sh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Shell/plugin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Shell/99-bottles-of-beer");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Shell/rvm.bash");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Shell/php.fcgi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Shell/script.zsh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Shell/string-chopping");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Shell/invalid-shebang.sh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Shell/sbt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Shell/valid-shebang.tool");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Shell/build.command");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Shell/script.bash");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Shell/default.env");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Shell/rbenv-sh-shell.sh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Shell/zsh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Shell/bash");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Shell/settime.cgi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Shell/filenames/zlogin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Shell/filenames/.zlogout");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Shell/filenames/.login");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Shell/filenames/.cshrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Shell/filenames/zprofile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/Shell/filenames/zlogout");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/Shell/filenames/.env");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/Shell/filenames/.profile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/Shell/filenames/.bashrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/Shell/filenames/.env.example");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/Shell/filenames/9fs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_29() {
        let path = Path::new(&"./samples/Shell/filenames/.zprofile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_30() {
        let path = Path::new(&"./samples/Shell/filenames/.zshrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_31() {
        let path = Path::new(&"./samples/Shell/filenames/zshenv");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_32() {
        let path = Path::new(&"./samples/Shell/filenames/bash_aliases");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_33() {
        let path = Path::new(&"./samples/Shell/filenames/.bash_aliases");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_34() {
        let path = Path::new(&"./samples/Shell/filenames/.bash_logout");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_35() {
        let path = Path::new(&"./samples/Shell/filenames/man");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_36() {
        let path = Path::new(&"./samples/Shell/filenames/profile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_37() {
        let path = Path::new(&"./samples/Shell/filenames/bash_logout");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_38() {
        let path = Path::new(&"./samples/Shell/filenames/login");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_39() {
        let path = Path::new(&"./samples/Shell/filenames/bash_profile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_40() {
        let path = Path::new(&"./samples/Shell/filenames/zshrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_41() {
        let path = Path::new(&"./samples/Shell/filenames/.bash_profile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_42() {
        let path = Path::new(&"./samples/Shell/filenames/bashrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_43() {
        let path = Path::new(&"./samples/Shell/filenames/gradlew");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_44() {
        let path = Path::new(&"./samples/Shell/filenames/.flaskenv");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_45() {
        let path = Path::new(&"./samples/Shell/filenames/cshrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_46() {
        let path = Path::new(&"./samples/Shell/filenames/PKGBUILD");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_47() {
        let path = Path::new(&"./samples/Shell/filenames/.zlogin");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_48() {
        let path = Path::new(&"./samples/Shell/filenames/.zshenv");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shell".to_string());
        assert_eq!(actual, expected);
    }
}

mod shellcheck_config {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ShellCheck Config/filenames/.shellcheckrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ShellCheck Config".to_string());
        assert_eq!(actual, expected);
    }
}

mod shellsession {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ShellSession/simple.sh-session");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ShellSession".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/ShellSession/gem-install.sh-session");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ShellSession".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/ShellSession/dollar.sh-session");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ShellSession".to_string());
        assert_eq!(actual, expected);
    }
}

mod shen {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Shen/json.shen");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shen".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Shen/html.shen");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shen".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Shen/graph.shen");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Shen".to_string());
        assert_eq!(actual, expected);
    }
}

mod sieve {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Sieve/dovecot-spamtest.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Sieve/dovecot-plus.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Sieve/dovecot-flagging.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Sieve/dovecot-vacation.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Sieve/dovecot-subaddress.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Sieve/dovecot-archive.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Sieve/dovecot-spam2.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Sieve/dovecot-spam1.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Sieve/wikipedia-example.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Sieve/dovecot-virustest.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Sieve/dovecot-headers.sieve");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Sieve".to_string());
        assert_eq!(actual, expected);
    }
}

mod singularity {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Singularity/filenames/Singularity");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Singularity".to_string());
        assert_eq!(actual, expected);
    }
}

mod slash {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Slash/brainfuck.sl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Slash".to_string());
        assert_eq!(actual, expected);
    }
}

mod slice {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Slice/testSlice01.ice");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Slice".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Slice/Test.ice");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Slice".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Slice/Murmur.ice");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Slice".to_string());
        assert_eq!(actual, expected);
    }
}

mod slim {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Slim/sample.slim");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Slim".to_string());
        assert_eq!(actual, expected);
    }
}

mod smpl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SmPL/atomic_as_refcounter.cocci");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SmPL".to_string());
        assert_eq!(actual, expected);
    }
}

mod smali {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Smali/ViewDragHelper.smali");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smali".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Smali/ActionBarDrawerToggle.smali");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smali".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Smali/DoodleMobileAnaylise.smali");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smali".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Smali/ModernAsyncTask.smali");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smali".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Smali/WbxmlSerializer.smali");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smali".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Smali/Subject.smali");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smali".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Smali/PenguinSprite.smali");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smali".to_string());
        assert_eq!(actual, expected);
    }
}

mod smalltalk {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Smalltalk/scriptWithPragma.st");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smalltalk".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Smalltalk/Dinner.st");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smalltalk".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Smalltalk/Collections.cs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smalltalk".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Smalltalk/baselineDependency.st");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smalltalk".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Smalltalk/TestBasic.st");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smalltalk".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Smalltalk/smallMethod.st");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smalltalk".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Smalltalk/renderSeasideExampleOn..st");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smalltalk".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Smalltalk/Booleans.cs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smalltalk".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Smalltalk/testSimpleChainMatches.st");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smalltalk".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Smalltalk/categories.st");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Smalltalk".to_string());
        assert_eq!(actual, expected);
    }
}

mod soong {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Soong/filenames/Android.bp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Soong".to_string());
        assert_eq!(actual, expected);
    }
}

mod sourcepawn {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SourcePawn/shavit.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SourcePawn".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SourcePawn/foo.sp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SourcePawn".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SourcePawn/bhopstats.inc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SourcePawn".to_string());
        assert_eq!(actual, expected);
    }
}

mod squirrel {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Squirrel/Squirrel.nut");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Squirrel".to_string());
        assert_eq!(actual, expected);
    }
}

mod stan {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Stan/dogs.stan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Stan/congress.stan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stan".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Stan/schools.stan");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stan".to_string());
        assert_eq!(actual, expected);
    }
}

mod standard_ml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Standard ML/RedBlackTree.fun");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Standard ML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Standard ML/Foo.ML");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Standard ML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Standard ML/Foo.sig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Standard ML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Standard ML/Foo.sml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Standard ML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Standard ML/main.fun");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Standard ML".to_string());
        assert_eq!(actual, expected);
    }
}

mod starlark {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Starlark/closure_js_binary.bzl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Starlark".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Starlark/filenames/BUILD");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Starlark".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Starlark/filenames/BUCK");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Starlark".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Starlark/filenames/WORKSPACE");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Starlark".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Starlark/filenames/Tiltfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Starlark".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Starlark/filenames/BUILD.bazel");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Starlark".to_string());
        assert_eq!(actual, expected);
    }
}

mod stata {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Stata/odkmeta.sthlp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stata".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Stata/regress_example.do");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stata".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Stata/hello.ado");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stata".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Stata/include.ihlp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stata".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Stata/tanh.mata");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stata".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Stata/limits.matah");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stata".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Stata/common.doh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stata".to_string());
        assert_eq!(actual, expected);
    }
}

mod stringtemplate {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/StringTemplate/ApiOverviewPage.st");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("StringTemplate".to_string());
        assert_eq!(actual, expected);
    }
}

mod stylus {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Stylus/demo.styl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Stylus".to_string());
        assert_eq!(actual, expected);
    }
}

mod subrip_text {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path =
            Path::new(&"./samples/SubRip Text/Adding.NCL.Language.S01E01.1080p.BluRay.x264.srt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SubRip Text".to_string());
        assert_eq!(actual, expected);
    }
}

mod sugarss {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SugarSS/sample.sss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SugarSS".to_string());
        assert_eq!(actual, expected);
    }
}

mod supercollider {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SuperCollider/WarpPreset.sc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SuperCollider".to_string());
        assert_eq!(actual, expected);
    }

    #[ignore]
    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SuperCollider/example.scd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SuperCollider".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SuperCollider/WarpTate.sc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SuperCollider".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/SuperCollider/WarpTrack.sc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SuperCollider".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/SuperCollider/WarpUtil.sc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SuperCollider".to_string());
        assert_eq!(actual, expected);
    }
}

mod svelte {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Svelte/TodoMVC.svelte");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Svelte".to_string());
        assert_eq!(actual, expected);
    }
}

mod swift {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Swift/section-31.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Swift/section-33.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Swift/section-5.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Swift/section-7.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Swift/section-55.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Swift/section-15.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Swift/section-43.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Swift/section-25.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Swift/section-73.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Swift/section-63.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Swift/section-37.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Swift/section-11.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Swift/section-3.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Swift/section-61.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Swift/section-45.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Swift/section-41.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Swift/section-47.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Swift/section-87.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Swift/section-85.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Swift/section-49.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Swift/section-23.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Swift/section-35.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Swift/section-75.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/Swift/section-19.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/Swift/section-29.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/Swift/section-57.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/Swift/section-53.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/Swift/section-71.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/Swift/section-27.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_29() {
        let path = Path::new(&"./samples/Swift/section-65.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_30() {
        let path = Path::new(&"./samples/Swift/section-9.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_31() {
        let path = Path::new(&"./samples/Swift/section-77.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_32() {
        let path = Path::new(&"./samples/Swift/section-83.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_33() {
        let path = Path::new(&"./samples/Swift/section-67.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_34() {
        let path = Path::new(&"./samples/Swift/section-13.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_35() {
        let path = Path::new(&"./samples/Swift/section-21.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_36() {
        let path = Path::new(&"./samples/Swift/section-69.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_37() {
        let path = Path::new(&"./samples/Swift/section-39.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_38() {
        let path = Path::new(&"./samples/Swift/section-79.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_39() {
        let path = Path::new(&"./samples/Swift/section-59.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_40() {
        let path = Path::new(&"./samples/Swift/section-51.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_41() {
        let path = Path::new(&"./samples/Swift/section-81.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_42() {
        let path = Path::new(&"./samples/Swift/section-17.swift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Swift".to_string());
        assert_eq!(actual, expected);
    }
}

mod systemverilog {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/SystemVerilog/endpoint_phy_wrapper.svh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SystemVerilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/SystemVerilog/priority_encoder.sv");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SystemVerilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/SystemVerilog/fifo.sv");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SystemVerilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/SystemVerilog/util.vh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("SystemVerilog".to_string());
        assert_eq!(actual, expected);
    }
}

mod ti_program {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/TI Program/srcgui.8xp.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TI Program".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/TI Program/srcfunc.8xp.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TI Program".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/TI Program/srcalpha.8xp.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TI Program".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/TI Program/srcsort.8xp.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TI Program".to_string());
        assert_eq!(actual, expected);
    }
}

mod tla {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/TLA/AsyncInterface.tla");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TLA".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/TLA/fifo.tla");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TLA".to_string());
        assert_eq!(actual, expected);
    }
}

mod toml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/TOML/filenames/poetry.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TOML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/TOML/filenames/Pipfile");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TOML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/TOML/filenames/Gopkg.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TOML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/TOML/filenames/Cargo.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TOML".to_string());
        assert_eq!(actual, expected);
    }
}

mod tsql {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/TSQL/cursor.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/TSQL/storedprocedure.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/TSQL/uniqueidentifier.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TSQL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/TSQL/logical.sql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TSQL".to_string());
        assert_eq!(actual, expected);
    }
}

mod tsv {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/TSV/input.tsv");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TSV".to_string());
        assert_eq!(actual, expected);
    }
}

mod tsx {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/TSX/import.tsx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TSX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/TSX/react-native.tsx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TSX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/TSX/triple-slash-reference.tsx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TSX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/TSX/require.tsx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TSX".to_string());
        assert_eq!(actual, expected);
    }
}

mod txl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/TXL/Cal.txl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TXL".to_string());
        assert_eq!(actual, expected);
    }
}

mod tcl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Tcl/stream-0.1.tm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Tcl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Tcl/xdgbasedir-0.3.tm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Tcl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Tcl/init.tcl.in");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Tcl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Tcl/filenames/owh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Tcl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Tcl/filenames/starfield");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Tcl".to_string());
        assert_eq!(actual, expected);
    }
}

mod tcsh {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Tcsh/regtest_nmmnest.csh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Tcsh".to_string());
        assert_eq!(actual, expected);
    }
}

mod tex {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/TeX/english.lbx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TeX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/TeX/beispiel.toc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TeX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/TeX/problemset.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TeX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/TeX/authortitle.cbx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TeX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/TeX/perl.toc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TeX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/TeX/reedthesis.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TeX".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/TeX/verbose.bbx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TeX".to_string());
        assert_eq!(actual, expected);
    }
}

mod tea {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Tea/foo.tea");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Tea".to_string());
        assert_eq!(actual, expected);
    }
}

mod terra {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Terra/arrayt.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Terra".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Terra/benchmark_nbody.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Terra".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Terra/arith.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Terra".to_string());
        assert_eq!(actual, expected);
    }
}

mod texinfo {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Texinfo/protocol-spec.texi");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Texinfo".to_string());
        assert_eq!(actual, expected);
    }
}

mod text {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Text/rmMonAnnCycLLT-help.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Text/min-help.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Text/tutor.nb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Text/aptitude-defaults.nb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Text/mac.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Text/tutor.no");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Text/foo.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Text/readme.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Text/zonalAve-help.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Text/messages.fr");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Text/filenames/keep.me");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Text/filenames/click.me");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Text/filenames/README.me");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Text/filenames/package.mask");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/Text/filenames/use.mask");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/Text/filenames/delete.me");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/Text/filenames/LICENSE.mysql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/Text/filenames/README.nss");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/Text/filenames/use.stable.mask");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/Text/filenames/readme.1st");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/Text/filenames/README.mysql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/Text/filenames/read.me");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/Text/filenames/test.me");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/Text/filenames/COPYING.regex");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/Text/filenames/package.use.stable.mask");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/Text/filenames/package.use.mask");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Text".to_string());
        assert_eq!(actual, expected);
    }
}

mod textmate_properties {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/TextMate Properties/filenames/.tm_properties");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TextMate Properties".to_string());
        assert_eq!(actual, expected);
    }
}

mod thrift {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Thrift/linguist.thrift");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Thrift".to_string());
        assert_eq!(actual, expected);
    }
}

mod turing {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Turing/start.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Turing".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Turing/BlockTower.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Turing".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Turing/simplegame.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Turing".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Turing/turing.t");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Turing".to_string());
        assert_eq!(actual, expected);
    }
}

mod turtle {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Turtle/rdf-syntax-grammar.ttl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Turtle".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Turtle/gnd-record.ttl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Turtle".to_string());
        assert_eq!(actual, expected);
    }
}

mod type_language {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Type Language/scheme.tl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Type Language".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Type Language/builtin.tl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Type Language".to_string());
        assert_eq!(actual, expected);
    }
}

mod typescript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/TypeScript/cache.ts");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TypeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/TypeScript/hello.ts");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TypeScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/TypeScript/classes.ts");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("TypeScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod unity3d_asset {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Unity3D Asset/Hover.anim");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Unity3D Asset".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Unity3D Asset/handFingers.mask");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Unity3D Asset".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Unity3D Asset/Tiles.meta");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Unity3D Asset".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Unity3D Asset/GapTile.mat");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Unity3D Asset".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Unity3D Asset/canvas_Fullscreen_Fader.prefab");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Unity3D Asset".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Unity3D Asset/TimeManager.asset");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Unity3D Asset".to_string());
        assert_eq!(actual, expected);
    }
}

mod unix_assembly {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Unix Assembly/support.S");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Unix Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Unix Assembly/gemm_kernel_1x4.S");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Unix Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Unix Assembly/hello.s");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Unix Assembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Unix Assembly/hello.ms");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Unix Assembly".to_string());
        assert_eq!(actual, expected);
    }
}

mod uno {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Uno/Pong.uno");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Uno".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Uno/PlayerPads.uno");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Uno".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Uno/TowerBlock.uno");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Uno".to_string());
        assert_eq!(actual, expected);
    }
}

mod unrealscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/UnrealScript/MutU2Weapons.uc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("UnrealScript".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/UnrealScript/US3HelloWorld.uc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("UnrealScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod urweb {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/UrWeb/parse.urs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("UrWeb".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/UrWeb/iso8601.ur");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("UrWeb".to_string());
        assert_eq!(actual, expected);
    }
}

mod v {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/V/loop.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("V".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/V/nbody.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("V".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/V/links_scraper.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("V".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/V/news_fetcher.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("V".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/V/rune.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("V".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/V/terminal_control.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("V".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/V/json.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("V".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/V/log.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("V".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/V/spectral.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("V".to_string());
        assert_eq!(actual, expected);
    }
}

mod vba {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/VBA/dictionary.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("VBA".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/VBA/cApplication.cls");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("VBA".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/VBA/procedures.vba");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("VBA".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/VBA/specs.bas");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("VBA".to_string());
        assert_eq!(actual, expected);
    }
}

mod vbscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/VBScript/v_Data_ArrayList.vbs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("VBScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod vcl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/VCL/varnish3_default.vcl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("VCL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/VCL/varnish2_default.vcl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("VCL".to_string());
        assert_eq!(actual, expected);
    }
}

mod vhdl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/VHDL/foo.vhd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("VHDL".to_string());
        assert_eq!(actual, expected);
    }
}

mod valve_data_format {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Valve Data Format/gameinfo.vdf");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Valve Data Format".to_string());
        assert_eq!(actual, expected);
    }
}

mod verilog {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Verilog/t_sqrt_pipelined.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Verilog/t_button_debounce.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Verilog/control.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Verilog/ram.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Verilog/sign_extender.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Verilog/pipeline_registers.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Verilog/sqrt_pipelined.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Verilog/sha-256-functions.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Verilog/ps2_mouse.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/Verilog/mux.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/Verilog/hex_display.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/Verilog/t_div_pipelined.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/Verilog/vga.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/Verilog/button_debounce.v");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Verilog".to_string());
        assert_eq!(actual, expected);
    }
}

mod vim_help_file {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Vim Help File/modeline.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Help File".to_string());
        assert_eq!(actual, expected);
    }
}

mod vim_script {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Vim Script/solarized.vim");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Vim Script/settings.vimrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Vim Script/textobj-rubyblock.vba");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Vim Script/todo.vmb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Vim Script/filenames/.vimrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Vim Script/filenames/.nvimrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/Vim Script/filenames/.gvimrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/Vim Script/filenames/.exrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/Vim Script/filenames/_vimrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Script".to_string());
        assert_eq!(actual, expected);
    }
}

mod vim_snippet {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Vim Snippet/vim.snippets");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Snippet".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Vim Snippet/vim.snip");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vim Snippet".to_string());
        assert_eq!(actual, expected);
    }
}

mod visual_basic__dot_net {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Visual Basic .NET/VBAllInOne.vb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Visual Basic .NET".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Visual Basic .NET/Index.vbhtml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Visual Basic .NET".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Visual Basic .NET/Module1.vb");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Visual Basic .NET".to_string());
        assert_eq!(actual, expected);
    }
}

mod volt {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Volt/tesla.volt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Volt".to_string());
        assert_eq!(actual, expected);
    }
}

mod vue {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Vue/basic.vue");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vue".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Vue/pre-processors.vue");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Vue".to_string());
        assert_eq!(actual, expected);
    }
}

mod wavefront_material {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Wavefront Material/shapes.mtl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wavefront Material".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Wavefront Material/ripple.mtl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wavefront Material".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Wavefront Material/spline.mtl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wavefront Material".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Wavefront Material/dice.mtl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wavefront Material".to_string());
        assert_eq!(actual, expected);
    }
}

mod wavefront_object {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Wavefront Object/ripple.obj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wavefront Object".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Wavefront Object/shapes.obj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wavefront Object".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Wavefront Object/spline.obj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wavefront Object".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Wavefront Object/random.obj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wavefront Object".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Wavefront Object/dice.obj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wavefront Object".to_string());
        assert_eq!(actual, expected);
    }
}

mod web_ontology_language {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Web Ontology Language/sample.owl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Web Ontology Language".to_string());
        assert_eq!(actual, expected);
    }
}

mod webassembly {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/WebAssembly/print.wat");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("WebAssembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/WebAssembly/add.wat");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("WebAssembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/WebAssembly/fibonacci.wat");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("WebAssembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/WebAssembly/imported-min.wast");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("WebAssembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/WebAssembly/local-cse.wast");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("WebAssembly".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(
            &"./samples/WebAssembly/remove-unused-brs_shrink-level=1_ignore-implicit-traps.wast",
        );
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("WebAssembly".to_string());
        assert_eq!(actual, expected);
    }
}

mod webidl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/WebIDL/Fetch.webidl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("WebIDL".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/WebIDL/AnimationEvent.webidl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("WebIDL".to_string());
        assert_eq!(actual, expected);
    }
}

mod webvtt {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/WebVTT/Godfather-Restaurant-Scene.vtt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("WebVTT".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/WebVTT/example.vtt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("WebVTT".to_string());
        assert_eq!(actual, expected);
    }
}

mod wget_config {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Wget Config/filenames/.wgetrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wget Config".to_string());
        assert_eq!(actual, expected);
    }
}

mod wikitext {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Wikitext/mediawiki.mediawiki");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wikitext".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Wikitext/README.wiki");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wikitext".to_string());
        assert_eq!(actual, expected);
    }
}

mod windows_registry_entries {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Windows Registry Entries/sample.reg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Windows Registry Entries".to_string());
        assert_eq!(actual, expected);
    }
}

mod wollok {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Wollok/piratas.wlk");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wollok".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Wollok/pepita.wlk");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Wollok".to_string());
        assert_eq!(actual, expected);
    }
}

mod world_of_warcraft_addon_data {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/World of Warcraft Addon Data/addon.toc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("World of Warcraft Addon Data".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/World of Warcraft Addon Data/lingua.toc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("World of Warcraft Addon Data".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/World of Warcraft Addon Data/linguist.toc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("World of Warcraft Addon Data".to_string());
        assert_eq!(actual, expected);
    }
}

mod x_bitmap {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/X BitMap/image.xbm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X BitMap".to_string());
        assert_eq!(actual, expected);
    }
}

mod x_font_directory_index {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/X Font Directory Index/filenames/fonts.alias");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X Font Directory Index".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/X Font Directory Index/filenames/encodings.dir");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X Font Directory Index".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/X Font Directory Index/filenames/fonts.dir");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X Font Directory Index".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/X Font Directory Index/filenames/fonts.scale");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X Font Directory Index".to_string());
        assert_eq!(actual, expected);
    }
}

mod x_pixmap {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/X PixMap/cc-public_domain_mark_white.pm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X PixMap".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/X PixMap/stick-unfocus.xpm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X PixMap".to_string());
        assert_eq!(actual, expected);
    }
}

mod x10 {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/X10/HeatTransfer_v1.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/X10/MontyPi.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/X10/NQueensPar.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/X10/KMeansSPMD.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/X10/HelloWorld.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/X10/QSort.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/X10/Histogram.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/X10/StructSpheres.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/X10/KMeansDist.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/X10/ArraySum.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/X10/HeatTransfer_v0.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/X10/NQueensDist.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/X10/Fibonacci.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/X10/KMeansDistPlh.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/X10/KMeans.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/X10/Cancellation.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/X10/Integrate.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/X10/HelloWholeWorld.x10");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("X10".to_string());
        assert_eq!(actual, expected);
    }
}

mod xc {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/XC/main.xc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XC".to_string());
        assert_eq!(actual, expected);
    }
}

mod xcompose {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/XCompose/filenames/XCompose");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XCompose".to_string());
        assert_eq!(actual, expected);
    }
}

mod xml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/XML/sample.csl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/XML/demo.hzp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/XML/real-estate.mjml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/XML/nproj-sample.nproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/XML/fsproj-sample.fsproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/XML/holobloc-sample.res");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/XML/CSharpVSPackage.vstemplate");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/XML/vcxproj-sample.vcxproj.filters");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/XML/MyApp.ux");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/XML/receiver.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/XML/translation_en3.ts");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/XML/example.ccproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/XML/obj_control.object.gmx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_13() {
        let path = Path::new(&"./samples/XML/pt_BR.xml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_14() {
        let path = Path::new(&"./samples/XML/sample.targets");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_15() {
        let path = Path::new(&"./samples/XML/chrome.natvis");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_16() {
        let path = Path::new(&"./samples/XML/Case.workflow");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_17() {
        let path = Path::new(&"./samples/XML/JSBrowser.jsproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_18() {
        let path = Path::new(&"./samples/XML/FXMLSample.fxml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_19() {
        let path = Path::new(&"./samples/XML/some-ideas.mm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_20() {
        let path = Path::new(&"./samples/XML/msbuild-example.proj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_21() {
        let path = Path::new(&"./samples/XML/vcxproj-sample.vcxproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_22() {
        let path = Path::new(&"./samples/XML/XmlIO.pluginspec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_23() {
        let path = Path::new(&"./samples/XML/source.extension.vsixmanifest");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_24() {
        let path = Path::new(&"./samples/XML/dependency-example.depproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_25() {
        let path = Path::new(&"./samples/XML/clouddef.csdef");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_26() {
        let path = Path::new(&"./samples/XML/vbproj-sample.vbproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_27() {
        let path = Path::new(&"./samples/XML/intellij.iml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_28() {
        let path = Path::new(&"./samples/XML/xhtml-struct-1.mod");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_29() {
        let path = Path::new(&"./samples/XML/Demo.sfproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_30() {
        let path = Path::new(&"./samples/XML/main.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_31() {
        let path = Path::new(&"./samples/XML/namespace-strict.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_32() {
        let path = Path::new(&"./samples/XML/psd-data.xmp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_33() {
        let path = Path::new(&"./samples/XML/MDM.adml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_34() {
        let path = Path::new(&"./samples/XML/oasis-table.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_35() {
        let path = Path::new(&"./samples/XML/module.ant");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_36() {
        let path = Path::new(&"./samples/XML/water.tsx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_37() {
        let path = Path::new(&"./samples/XML/System.Buffers.pkgproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_38() {
        let path = Path::new(&"./samples/XML/pt_BR.ts");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_39() {
        let path = Path::new(&"./samples/XML/01_top.ncl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_40() {
        let path = Path::new(&"./samples/XML/point-3.2.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_41() {
        let path = Path::new(&"./samples/XML/example-sharedproj.shproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_42() {
        let path = Path::new(&"./samples/XML/GMOculus.project.gmx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_43() {
        let path = Path::new(&"./samples/XML/battlescribe.gst");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_44() {
        let path = Path::new(&"./samples/XML/src.builds");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_45() {
        let path = Path::new(&"./samples/XML/csproj-sample.csproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_46() {
        let path = Path::new(&"./samples/XML/Example.mdpolicy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_47() {
        let path = Path::new(&"./samples/XML/module.ivy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_48() {
        let path = Path::new(&"./samples/XML/HITSP_C32.sch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_49() {
        let path = Path::new(&"./samples/XML/Strings.resx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_50() {
        let path = Path::new(&"./samples/XML/configdef.cscfg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_51() {
        let path = Path::new(&"./samples/XML/Storyboard.storyboard");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_52() {
        let path = Path::new(&"./samples/XML/MDM.admx");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_53() {
        let path = Path::new(&"./samples/XML/point-3.1.gml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_54() {
        let path = Path::new(&"./samples/XML/racoon.mjml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_55() {
        let path = Path::new(&"./samples/XML/libsomething.dll.config");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_56() {
        let path = Path::new(&"./samples/XML/sample.nuspec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_57() {
        let path = Path::new(&"./samples/XML/NDepends_Example.ndproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_58() {
        let path = Path::new(&"./samples/XML/net_docfile.xml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_59() {
        let path = Path::new(&"./samples/XML/phpunit.xml.dist");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_60() {
        let path = Path::new(&"./samples/XML/wixdemo.wixproj");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_61() {
        let path = Path::new(&"./samples/XML/cloudconfig.cscfg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_62() {
        let path = Path::new(&"./samples/XML/tei-odd-sample.odd");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_63() {
        let path = Path::new(&"./samples/XML/xquery-tutorial.xspec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_64() {
        let path = Path::new(&"./samples/XML/Application.xib");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_65() {
        let path = Path::new(&"./samples/XML/WebElement.rs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_66() {
        let path = Path::new(&"./samples/XML/MainView.ux");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_67() {
        let path = Path::new(&"./samples/XML/Default.props");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_68() {
        let path = Path::new(&"./samples/XML/filenames/.cproject");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML".to_string());
        assert_eq!(actual, expected);
    }
}

mod xml_property_list {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/XML Property List/widget.stTheme");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML Property List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/XML Property List/Folding.tmPreferences");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML Property List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/XML Property List/Man.tmLanguage");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML Property List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/XML Property List/SpaceCadet.tmTheme");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML Property List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/XML Property List/Completion.tmCommand");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML Property List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/XML Property List/And.tmSnippet");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML Property List".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/XML Property List/info.plist");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XML Property List".to_string());
        assert_eq!(actual, expected);
    }
}

mod xpages {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/XPages/navbar.xsp.metadata");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XPages".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/XPages/navbar.xsp-config");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XPages".to_string());
        assert_eq!(actual, expected);
    }
}

mod xproc {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/XProc/xproc.xpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XProc".to_string());
        assert_eq!(actual, expected);
    }
}

mod xquery {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/XQuery/xproc.xqm");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XQuery".to_string());
        assert_eq!(actual, expected);
    }
}

mod xs {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/XS/CommonMark.xs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XS".to_string());
        assert_eq!(actual, expected);
    }
}

mod xslt {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/XSLT/test.xslt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("XSLT".to_string());
        assert_eq!(actual, expected);
    }
}

mod xojo {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Xojo/MainMenuBar.xojo_menu");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Xojo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Xojo/BillingReport.xojo_report");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Xojo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Xojo/database.xojo_script");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Xojo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/Xojo/MyToolbar.xojo_toolbar");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Xojo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/Xojo/App.xojo_code");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Xojo".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/Xojo/Window1.xojo_window");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Xojo".to_string());
        assert_eq!(actual, expected);
    }
}

mod xonsh {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Xonsh/rever.xsh");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Xonsh".to_string());
        assert_eq!(actual, expected);
    }
}

mod xtend {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Xtend/BasicExpressions.xtend");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Xtend".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Xtend/Movies.xtend");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Xtend".to_string());
        assert_eq!(actual, expected);
    }
}

mod yaml {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/YAML/Ansible.YAML-tmLanguage");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/YAML/coredns.yaml.sed");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/YAML/source.r-console.syntax");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/YAML/database.yml.mysql");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let path = Path::new(&"./samples/YAML/expected-floating-point-literal.mir");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let path = Path::new(&"./samples/YAML/HexInspect.sublime-syntax");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let path = Path::new(&"./samples/YAML/vcr_cassette.yml");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let path = Path::new(&"./samples/YAML/filenames/.gemrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_8() {
        let path = Path::new(&"./samples/YAML/filenames/.clang-tidy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_9() {
        let path = Path::new(&"./samples/YAML/filenames/CITATION.cff");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_10() {
        let path = Path::new(&"./samples/YAML/filenames/.clang-format");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_11() {
        let path = Path::new(&"./samples/YAML/filenames/yarn.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_12() {
        let path = Path::new(&"./samples/YAML/filenames/glide.lock");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YAML".to_string());
        assert_eq!(actual, expected);
    }
}

mod yang {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/YANG/sfc-lisp-impl.yang");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YANG".to_string());
        assert_eq!(actual, expected);
    }
}

mod yara {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/YARA/true.yar");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YARA".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/YARA/OfExample.yar");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YARA".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/YARA/example.yara");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YARA".to_string());
        assert_eq!(actual, expected);
    }
}

mod yasnippet {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/YASnippet/font-face.yasnippet");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YASnippet".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/YASnippet/stdin.yasnippet");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("YASnippet".to_string());
        assert_eq!(actual, expected);
    }
}

mod yacc {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Yacc/calc.yy");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Yacc".to_string());
        assert_eq!(actual, expected);
    }
}

mod zap {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ZAP/planetfall.zap");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ZAP".to_string());
        assert_eq!(actual, expected);
    }
}

mod zil {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ZIL/misc.zil");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ZIL".to_string());
        assert_eq!(actual, expected);
    }
}

mod zeek {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Zeek/main.bro");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Zeek".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Zeek/example.zeek");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Zeek".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Zeek/main.zeek");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Zeek".to_string());
        assert_eq!(actual, expected);
    }
}

mod zenscript {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/ZenScript/sample.zs");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("ZenScript".to_string());
        assert_eq!(actual, expected);
    }
}

mod zephir {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Zephir/Cblock.zep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Zephir".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Zephir/Router.zep");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Zephir".to_string());
        assert_eq!(actual, expected);
    }
}

mod zig {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Zig/guess_number.zig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Zig".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/Zig/cat.zig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Zig".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/Zig/hello.zig");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Zig".to_string());
        assert_eq!(actual, expected);
    }
}

mod zimpl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/Zimpl/sample.zmpl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("Zimpl".to_string());
        assert_eq!(actual, expected);
    }
}

mod curl_config {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/cURL Config/filenames/.curlrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("cURL Config".to_string());
        assert_eq!(actual, expected);
    }
}

mod desktop {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/desktop/example.desktop");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("desktop".to_string());
        assert_eq!(actual, expected);
    }
}

mod dircolors {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/dircolors/sample.dircolors");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("dircolors".to_string());
        assert_eq!(actual, expected);
    }
}

mod ec {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/eC/Designer.ec");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("eC".to_string());
        assert_eq!(actual, expected);
    }
}

mod edn {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/edn/bigger-than-pluto.edn");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("edn".to_string());
        assert_eq!(actual, expected);
    }
}

mod fish {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/fish/funced.fish");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("fish".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/fish/config.fish");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("fish".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/fish/eval.fish");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("fish".to_string());
        assert_eq!(actual, expected);
    }
}

mod jq {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/jq/sample.jq");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("jq".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/jq/builtin.jq");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("jq".to_string());
        assert_eq!(actual, expected);
    }
}

mod mirc_script {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/mIRC Script/torncity-tsspy.mrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("mIRC Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/mIRC Script/AutoHostmeBot.mrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("mIRC Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/mIRC Script/torncity-tcbot.mrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("mIRC Script".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let path = Path::new(&"./samples/mIRC Script/torncity-apiprofile.mrc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("mIRC Script".to_string());
        assert_eq!(actual, expected);
    }
}

mod mcfunction {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/mcfunction/showcase.mcfunction");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("mcfunction".to_string());
        assert_eq!(actual, expected);
    }
}

mod nanorc {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/nanorc/nanorc.nanorc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("nanorc".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/nanorc/filenames/.nanorc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("nanorc".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/nanorc/filenames/nanorc");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("nanorc".to_string());
        assert_eq!(actual, expected);
    }
}

mod q {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/q/ml.q");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("q".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/q/tq.q");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("q".to_string());
        assert_eq!(actual, expected);
    }
}

mod restructuredtext {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/reStructuredText/HACKING.rst.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("reStructuredText".to_string());
        assert_eq!(actual, expected);
    }
}

mod robots_dot_txt {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/robots.txt/filenames/robots.txt");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("robots.txt".to_string());
        assert_eq!(actual, expected);
    }
}

mod sed {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/sed/hanoi.sed");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("sed".to_string());
        assert_eq!(actual, expected);
    }
}

mod wdl {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/wdl/hello.wdl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("wdl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/wdl/passingfiles.wdl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("wdl".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/wdl/ifs_in_scatters.wdl");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("wdl".to_string());
        assert_eq!(actual, expected);
    }
}

mod wisp {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/wisp/intro.wisp");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("wisp".to_string());
        assert_eq!(actual, expected);
    }
}

mod xbase {
    use file_expert::guess;
    use file_expert::Guess;
    use std::path::Path;

    #[test]
    fn test_0() {
        let path = Path::new(&"./samples/xBase/sample.ch");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("xBase".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let path = Path::new(&"./samples/xBase/sample.prw");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("xBase".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let path = Path::new(&"./samples/xBase/sample.prg");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("xBase".to_string());
        assert_eq!(actual, expected);
    }
}
