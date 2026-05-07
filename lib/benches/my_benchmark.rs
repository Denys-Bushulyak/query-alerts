use criterion::{Criterion, criterion_group, criterion_main};
use prewave_test_task_lib::{
    algorithms::with_regex,
    dtos::{AlertDto, QueryTermDto},
    query,
};

fn criterion_benchmark(c: &mut Criterion) {
    let alert_dtos: Vec<AlertDto> = serde_json::from_str(r#"[{"id":"uktt4245t3t4334","contents":[{"text":"Sales Punching faking sales numbers Hey, SEC, look at Tesla","type":"text","language":"en"},{"text":"Tesla Full Self Driving: New update now drives thru Green Lights and auto stops at Red Lights and Stop Signs without interaction. Full Self Self Driving everywhere getting oh so close. Here’s to the future","type":"text","language":"en"}],"date":"2026-05-06T21:03:00.227Z","inputType":"tweet"},{"id":"rtct54gf34343","contents":[{"text":"Another disaster for Ferrari.","type":"text","language":"ên"}],"date":"2026-05-06T20:45:14.765Z","inputType":"tweet"},{"id":"hz5n855393tn","contents":[{"text":"MÜNCHEN (dpa-AFX) - Die IG Metall erwartet einen heißen Herbst mit vielen Auseinandersetzungen um Jobs in Deutschland. Es wird um viele Arbeitsplätze gehen, sagte Vorstandsmitglied und Hauptkassierer Jürgen Kerner am Mittwochabend in München. Alleine in den Branchen, für die die Gewerkschaft zuständig sei, stünden ungefähr 300 000 Jobs im Feuer. Der größte Bereich dabei seien die Automobilindustrie und deren Zulieferer, doch auch anderen Branchen seien betroffen.","type":"short","language":"de"}],"date":"2026-05-06T20:44:38.374Z","inputType":"link"},{"id":"trg5g4tb4tg4red","contents":[{"text":"@En1Buena Ojalá lo logre eh, y una oferta de Ferrari es inigualable pero la veo fea","type":"text","language":"es"}],"date":"2026-05-06T20:20:02.580Z","inputType":"tweet"},{"id":"6gbujhu89786","contents":[{"text":"Wolfgang Lemb, ig metall Germany stands in solidarity with #StrikeForBlackLives","type":"text","language":"de"}],"date":"2026-05-06T20:56:23.987Z","inputType":"tweet"}]"#
   ).unwrap();
    let alerts = alert_dtos
        .into_iter()
        .map(|a| a.try_into().unwrap())
        .collect::<Vec<_>>();

    let term_dtos: Vec<QueryTermDto>  = serde_json::from_str(r#"[{"id":101,"target":1,"text":"IG Metall","language":"de","keepOrder":true},{"id":102,"target":1,"text":"IG Metall","language":"en","keepOrder":true},{"id":103,"target":1,"text":"Industriegewerkschaft Metall","language":"de","keepOrder":false},{"id":201,"target":2,"text":"Arbeitsplatz","language":"de","keepOrder":true},{"id":202,"target":2,"text":"Arbeitsplätze","language":"de","keepOrder":true},{"id":203,"target":2,"text":"job","language":"en","keepOrder":true},{"id":204,"target":2,"text":"jobs","language":"en","keepOrder":true},{"id":301,"target":3,"text":"pollution","language":"en","keepOrder":true},{"id":302,"target":3,"text":"inquinante","language":"it","keepOrder":true},{"id":401,"target":4,"text":"lithium","language":"en","keepOrder":true},{"id":501,"target":5,"text":"close","language":"en","keepOrder":true},{"id":502,"target":5,"text":"closure","language":"en","keepOrder":true},{"id":503,"target":5,"text":"closing","language":"en","keepOrder":true},{"id":601,"target":6,"text":"Tesla","language":"en","keepOrder":true},{"id":602,"target":6,"text":"Tesla","language":"de","keepOrder":true},{"id":603,"target":6,"text":"Tesla","language":"it","keepOrder":true},{"id":604,"target":6,"text":"Tesla","language":"es","keepOrder":true},{"id":701,"target":7,"text":"Yuasa","language":"en","keepOrder":true},{"id":801,"target":8,"text":"minimal involvement","language":"en","keepOrder":false},{"id":901,"target":9,"text":"coronavirus","language":"en","keepOrder":true},{"id":902,"target":9,"text":"coronavirus","language":"es","keepOrder":true},{"id":903,"target":9,"text":"covid-19","language":"en","keepOrder":true},{"id":904,"target":9,"text":"covid-19","language":"es","keepOrder":true},{"id":1001,"target":10,"text":"muerte","language":"es","keepOrder":true},{"id":1101,"target":11,"text":"fake","language":"en","keepOrder":true},{"id":1102,"target":11,"text":"faking","language":"en","keepOrder":true},{"id":1201,"target":12,"text":"dp world","language":"en","keepOrder":true},{"id":1202,"target":12,"text":"dp world","language":"es","keepOrder":true},{"id":1203,"target":12,"text":"dp world","language":"de","keepOrder":true},{"id":1204,"target":12,"text":"Dubai Ports World","language":"en","keepOrder":true}]"#).unwrap();

    let terms = term_dtos
        .into_iter()
        .map(|t| t.try_into().unwrap())
        .collect::<Vec<_>>();

    c.bench_function("query", |b| {
        b.iter(|| query(&alerts, with_regex(&terms)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
