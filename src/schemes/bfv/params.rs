use std::collections::HashMap;

use lazy_static::lazy_static;
use rug::Integer;
use crate::schemes::bfv::params::PARAMS::*;

#[derive(Clone)]
pub struct Params{
    pub (crate) s: f64, 
    pub (crate) n: usize, 
    pub (crate) p: Integer,  
    pub (crate) t: Integer,   
    pub (crate) rp: Integer,
    pub (crate) l: usize,
    pub (crate) w: Integer,
    pub (crate) phi: Integer,
    pub (crate) w_inv: Integer,
    pub (crate) phi_inv: Integer,
    pub (crate) precision: usize, 
}

impl Params{
    pub fn p(&self) -> Integer{
        self.p.clone()
    }

    pub fn n(&self) -> usize{
        self.n.clone()
    }

    pub fn prec(&self) -> usize{
        self.precision.clone()
    }

    pub fn w(&self) -> Integer{
        self.w.clone()
    }
    pub fn w_inv(&self) -> Integer{
        self.w_inv.clone()
    }
    pub fn phi(&self) -> Integer{
        self.phi.clone()
    }
    pub fn phi_inv(&self) -> Integer{
        self.phi_inv.clone()   
    }
}

#[derive(Clone, Debug)]
pub enum PARAMS{
    /// D = 1, log(p) = 30, t = 1024, n = 1024
    RlweParams1, 

    /// D = 2, log(p) = 58, t = 1024, n = 2048
    RlweParams2,

    /// D = 3, log(p) = 91, t = 1024, n = 2048
    RlweParams3, 

    /// D = 3, log(p) = 95, t = 1024, n = 4096
    RlweParams4,

    /// D = 4, log(p) = 130, t = 1024, n = 4096  
    RlweParams5, 

    /// D = 5, log(p) = 165, t = 1024, n = 4096 
    RlweParams6, 

    /// D = 5, log(p) = 171, t = 1024, n = 8192
    RlweParams7, 

    /// D = 10, log(p) = 354, t = 1024, n = 8192 
    RlweParams8, 

    /// D = 10, log(p) = 368, t = 1024, n = 16384
    RlweParams9, 

    /// D = 15, log(p) = 558, t = 1024, n = 16384 
    RlweParams10,

    /// D = 32, log(p) = 1298, t = 1024, n = 65536
    RlweParams12,

    /// D = 64, log(p) = 2705, t = 1024, n = 131072
    RlweParams11,

}
impl PARAMS{
    pub fn get_all() -> &'static [PARAMS]{
        &[
         RlweParams1, RlweParams2, RlweParams3, RlweParams4, RlweParams5, RlweParams6, RlweParams7, RlweParams8, RlweParams9, RlweParams10, RlweParams11, RlweParams12
        ]
    }
}

lazy_static!{
    static ref PARAMSETS: HashMap<&'static str, Params> = {
        let mut m = HashMap::new();

        let n = 1024; 
        let p = Integer::from(1061093377); 
        m.insert(
            "PARAMS1",
            Params {
                s: 8.0,
                n,
                p: p.clone(),
                t: Integer::from(1024),
                rp: p.clone().sqrt(),
                l: 2,
                w: Integer::from_str_radix("591137462", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("541153008", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("248390058", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("457488391", 10).expect("Cannot convert"), 
                precision: 64,
            },
            
        );

        let n = 2048; 
        let p = Integer::from(144115188076060673u64); 
        m.insert(
            "PARAMS2", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(),
                t: Integer::from(1024), 
                rp: p.clone().sqrt(),
                l: 2,
                w: Integer::from_str_radix("65170666404517193", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("38857149756300966", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("43860450731918522", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("136483537181756498", 10).expect("Cannot convert"),
                precision: 128, 
            }
        );
    
        let n = 2048; 
        let p = Integer::from_str_radix("2475880078570760549798268929",10).expect("Can't convert"); 
        m.insert(
            "PARAMS3", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: Integer::from(1024), 
                l: 10,
                w: Integer::from_str_radix("1627216933939800185070529534", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("1629869213090559976669113882", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("410842127805060305356328975", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("2025198391817557901322880644", 10).expect("Cannot convert"),
                precision: 256,
            }
        );

        let n = 4096; 
        let p = Integer::from_str_radix("39614081257132168796772007937", 10).expect("Cannot convert");
        m.insert(
            "PARAMS4", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: p.clone().sqrt(), 
                l: 2,
                w: Integer::from_str_radix("29457459092457819440271331685", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("4041360221600741923552860650", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("14404707505321999315413376944", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("18125588093367420813600092229", 10).expect("Cannot convert"),
                precision: 256,
            }
        );


        let n = 4096; 
        let p = Integer::from_str_radix("1361129467683753853853498429727072952321", 10).expect("Cannot convert");
        m.insert(
            "PARAMS5", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: p.clone().sqrt(), 
                l: 2,
                w: Integer::from_str_radix("699867356066028974742523726596515437409", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("288018062307076292221397865785279904932", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("292872717768032452906080992143075293280", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("841717877754518279107607783726578295324", 10).expect("Cannot convert"),
                precision: 512,
            }
        );

        let n = 4096; 
        let p = Integer::from_str_radix("46768052394588893382517914646921056628989841391617", 10).expect("Cannot convert"); 
        m.insert(
            "PARAMS6", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: p.clone().sqrt(), 
                l: 2,
                w: Integer::from_str_radix("28295734718647964331169717989933245896800811535333", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("18718979040386876872194208749966208491577952197368", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("22433533497085980182093608927750428278606578394105", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("629333450482215425169192060871240340146861661369", 10).expect("Cannot convert"),
                precision: 512,
            }
        );

        let n = 8192; 
        let p = Integer::from_str_radix("2993155353253689176481146537402947624255349848096769", 10).expect("Cannot convert"); 
        m.insert(
            "PARAMS7", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: p.clone().sqrt(), 
                l: 2,
                w: Integer::from_str_radix("336575187069724100844740872041130820398044724560490", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("1361043815560145881988679868600443300742269435413680", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("2374558713467120696889422412651599656693237594275430", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("260369858953212787406033233614944285118797814389754", 10).expect("Cannot convert"),
                precision: 512,
            }
        );
        let n = 8192;
        let p = Integer::from_str_radix("36695977855841144185773134324833391052745039826692497979801421430190766017415756929120296849762010990379009", 10).expect("Cannot convert");
        m.insert(
            "PARAMS8", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: p.clone().sqrt(), 
                l: 2,
                w: Integer::from_str_radix("7825181319709742232874141750598665460812878781304270007126535062042667119874069679995848262680968593386775", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("26170024533603042369305076975196655140107157083043684297159962276391832551665449584613789446670274084636121", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("17343126193259999160138838087920901169553393690556954745755974753243768748492483851421350403809913415549061", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("2445182749325392648408307827140537821858578943089677441148739007848812423843383156272728473112755170224676", 10).expect("Cannot convert"),
                precision: 768,
            }
        );

        let n = 16384;
        let p = Integer::from_str_radix("601226901190101306339707032778070279008174732520529886901066488712245510429339761526706943586500787976175583233", 10).expect("Cannot convert");
        
        m.insert(
            "PARAMS9", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: p.clone().sqrt(), 
                l: 2,
                w: Integer::from_str_radix("500867507409570607888704713581915139193485795428604636671384662582469928844307205726974657831834734217228923536", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("559465819912380557218964385075214318766955790860680139760983442359880435520914313415522889223586576225274665680", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("298887255939541710953725599994887409242174862896570983494632330275622619877095170121254125516563698514279547357", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("170970791447608759794625323064560028008846454461879025749520883671062159411931697560062907662178451430865723616", 10).expect("Cannot convert"),
                precision: 768,
            }
        );
        let n = 16384;
        let p = Integer::from_str_radix("943490606205385338060388645247067222729230305104110107094051575061406040598037213021531681294414691885367093757690961224942646157481198158140358562858174010912350732289", 10).expect("Cannot convert");
        m.insert(
            "PARAMS10", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: p.clone().sqrt(), 
                l: 2,
                w: Integer::from_str_radix("806938197572492384364504003540853878476988513933585364918218036613755157772878289461809361019765563408742683841708190102572088167895212748764946080623098350542201639451", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("845001705591929506224787688507184591335793239305131347344113796286717677815100122606606971420742128960084434208666825073427379183265171698276786992083115247171799998428", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("226458952803768183590597372596910082288575828662950078699401107385226746627785816239774895538507513069058716646799135998851029338821343062535837565815000422530592002751", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("878183683315365658582840836267570296883341505757401616771166125813433483986095081132008795107296309066967700356048481361801750262189285157672325465608885531987623246296", 10).expect("Cannot convert"),
                precision: 1152,
            }
        );

        let n = 65536;
        let p = Integer::from_str_radix("5456753954476208465864420400753061678869518613094966128971146775968406862390026723924240992426956782222663543136862081012928907181201012769758855354477351148151966012218655556644730752027372316510366948680107195740556454604333896888202035925844366611831769732382017888472749354531294074459245684662426036782795704082440137488045669345516825099044415534392245957801919491895828502926660206593", 10).expect("Cannot convert");
        m.insert(
            "PARAMS11", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: p.clone().sqrt(), 
                l: 2,
                w: Integer::from_str_radix("163168093699788210414750014135139560819970217222805027519080084218737472916999240071490564152407817514233203816242359423665059082219251200167791378904978465044422482024929443458343410540901387427875233891479486191786824652000657923100669908287427722062076122323979594140696341239500554196671213596327136219349353260896652504883935800868042741147574203213729396841484254558027827377917088412", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("757301536100998893323389465133513675254900203343909663067206578132059471194381445453519330571969317438392950107512040460765856949276900124475302535488551409128511899422901545439032279737390706695117039431891631934641724827268432168962051882850910047995191276519688881390554488034463639851181552710098301829579714027777674488959441190717887765099805252740737981713061669789357411809267708278", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("5184425187987606665750118059456455843836087058049596045691062396474272397257962139234053145188868048703927198052016275157193152647411517635455901168234840151868416892291685416739833421411159389209893083882512593771617675171256213502439925756249749362175425041883648531384269633950246879962198637852500644785100789445277696154563300654265861098588213777892060583683556758408589182976507462235", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("1466469643177560196724054014407203011871753009161657810260729960942775808333946524830024210125607288146859291876453669019232359724559042672018116363585418407033369070566939785555309970072176311496980491154281536284362162473309357798155455563916176944995003833621701529732025545580464052475502351990064359299297119168721620328772176416708059841919693516340667361860173447685827764920295819739", 10).expect("Cannot convert"),
                precision: 2688,
            }
        );

        let n = 131072;
        let p = Integer::from_str_radix("19325835167621765504215614612828412810670380182741813945362906260263582076873558013706075958043835443006690242316345928450084829183372668734444015413891231453817678477882751191066418548932668987779589287551053358849760163760036282070559234584792839505418688119239570841846335455858760472771048955629636387110965634275348054749486904358889843520172228385612340756906261312885423829202759981547912826976735872178110921489593244067807115547761654661785666040066804034735427381057748458934997215131221881556417001127575881486324555588715174586664402189745668735169431786104358215534044059708015571386507963928134564948797588820386180488824714070129291507181269328895942814464950987898180093004396845008489173329487427376304321022207500830400329030924863610496382512270820434798327290649742013373139310538540522971332609", 10).expect("Cannot convert");
        m.insert(
            "PARAMS12", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: p.clone().sqrt(), 
                l: 2,
                w: Integer::from_str_radix("17440762205927312060436539630579257596295970737843090304826008688118597903890081368366308618615714726860062787819738728445242470358177194607137277225850853144898616043082705993308372870059054018492859153673508056507125231158184717397087791103848049584911188088403628979266981049838654334504024721498228001416773308402299578465575091994662245981103596567647509711289489833557181330889508678171836201259458766030192206965199075814102521240888898181124701812679764851536551832036585354651525311779627657735581644862410341844529602310739964892659205218937472817412497681797745121562923007820747664405812186165789875847170450456559360250148518654955832188130491925667843268324485457262882079381652630261394812005870736509879387676042463498755903564189582420149956775000761128679798492039162145074847397954077397570016464", 10).expect("Cannot convert"),
                w_inv: Integer::from_str_radix("7942665784424006451700206197206969649762680338638267945708229266484296091219465552986217716554074446380309707806500780656398838054342344995821422780169683397885273997044326430378746122784833503401244763823730415125075643323005663227633022639408618506947272853771193216557664905044615839324631633842969430932367838481806217636692313336035669941167390960171653995831669478461469509528190589334922200993155422620213378692111458222348067206799001617777982803987069512109520387184539742485063558393097554499360205291632466521441995853598200499785944689276054271497686330356768531749102760819621314884705250959998321461775269918773029131230230898310474695722012338273375755294176614674443488642089701237599159661849835401514774584307707073231449251757024025376101056761118424344870193352452280069389769121178276515341038", 10).expect("Cannot convert"),
                phi: Integer::from_str_radix("1329706910588272897485121887670766359140363370866608929683296873003950241818237805526392958858288778774543469104684195271070732285604343607862222434874679601248891708016239108840074828541914470559958241394458712583426442581815378640677930216302184217233782821518505714085300299745040455671913489165656242446193312276722020697313790574494645405149506851693627313679409481620436012676365982656445870781597899211631844480719138287719771840005013148295057422407194065372736302946728824899260773606063860722403204884127820237598354274003890022791405905821545032753684635241481420843439285920990906347380931884033201964874394727525932528298175021181270723625301051486250337304146737103471361891699991470787198178097936869264468777353618275436530398454783574710691475374875847946297282330633939194841477187071290928214803", 10).expect("Cannot convert"),
                phi_inv: Integer::from_str_radix("8406881990809355130629842503597012656726341393477433614694811287741562003346543294945442114570507271187093130201776122560256541036235064161679211864616305368250283586303450333160625427224765035828043554515535794256700551604264346152306859831580000936550901582779131574569702538439284409172760653285537886581302018407880149800298920134619479968338235579072783315205503372992155846742100683457387154908674643227641049649954759394658774068116647910633181060020239679015919532274703289611498636123631484082905409217149545142345474389291269548584700855571856142674477865276035630938306439370358076786094647298190392125449349897049436660891505981247675918333683987598083504670618157450261734203887450328005784051654782334819126522179589428052940809898696651324291946259937937313525412475209180750783422456323193686299527", 10).expect("Cannot convert"),
                precision: 5632,
            }
        );    
        m
    };
}
pub fn get_params(param: PARAMS) -> Params{
    let name = match param {
        PARAMS::RlweParams1 => "PARAMS1", 
        PARAMS::RlweParams2 => "PARAMS2",
        PARAMS::RlweParams3 => "PARAMS3",
        PARAMS::RlweParams4 => "PARAMS4",
        PARAMS::RlweParams5 => "PARAMS5", 
        PARAMS::RlweParams6 => "PARAMS6", 
        PARAMS::RlweParams7 => "PARAMS7", 
        PARAMS::RlweParams8 => "PARAMS8", 
        PARAMS::RlweParams9 => "PARAMS9", 
        PARAMS::RlweParams10 => "PARAMS10", 
        PARAMS::RlweParams11 => "PARAMS11", 
        PARAMS::RlweParams12 => "PARAMS12",
    };
    PARAMSETS.get(name).cloned().expect("No such params")
}