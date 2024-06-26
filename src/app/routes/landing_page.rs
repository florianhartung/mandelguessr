use leptos::{component, create_resource, view, IntoView};
use leptos_router::A;

#[component]
pub fn LandingPage() -> impl IntoView {
    let link_class = "text-blue-700 underline";

    view! {
        <div class="w-full h-80 p-12 relative"> // header
            <img src="/header.png" class="w-full h-80 rounded-lg brightness-50"/>
            <div class="h-full w-full absolute top-0 left-0 flex flex-col items-center justify-center">
                <h1 class="mt-2 text-8xl mb-4 text-white font-bold">"MandelGuessr"</h1>
                <p class="text-2xl text-white font-bold">"It's like Geoguessr except it's 💩"</p>
                <div class="flex flex-row space-x-4 mt-8">
                    <A href="/WebEng/Projekt/login"><button class="rounded-full bg-[#600070] text-white text-xl font-bold py-2 px-4">Anmelden</button></A>
                    <A href="/WebEng/Projekt/leaderboard"><button class="rounded-full bg-[#600070] text-white text-xl font-bold py-2 px-4">Rangliste</button></A>
                </div>
            </div>
        </div>

        <div class="text-white font-md p-4 flex flex-col space-y-2 mt-40">
            <h2 class="font-bold font-xl">Konzept</h2>
            <h3 class="font-bold underline">Anforderungen</h3>
            <ul class="list-disc">
                <li>"Effizientes, platformunabhängiges Rendering eines interaktiven Mandelbrot-Sets im Browser"</li>
                <li>"Spiel, bei welchem eine zufälliger interaktiver Ausschnitt aus dem Mandelbrot-Set gezeigt wird, welcher wiedergefunden werden muss"</li>
                <li>"Benutzersystem sodass Rangliste und evtl. Erweiterungen in der Zukunft möglich sind"</li>
                <li>"Login- und Registrierungsseite"</li>
            </ul>
            <h3 class="font-bold underline">Design</h3>
            <div class="flex flex-row space-x-4 w-4/5">
                <img src="/mockup_landingpage.png" class="w-[25vw]"/>
                <img src="/mockup_gamepage.png" class="w-[25vw]"/>
                <img src="/mockup_postgame.png" class="w-[25vw]"/>
            </div>
            <p>
                Notiz: Für die Login- und Registrierungsseiten wurden keine Mockups erstellt, da diese in dieser Anwendung nur einen rein funktionalen und simplen Zweck erfüllen.
            </p>

            <img src="/flowchart.drawio.png" class="w-[40vw]"/>


            <h3 class="font-bold underline">Implementierung</h3>
            <p>
                Die Implementierung lässt sich in zwei Teile aufteilen:
                <ol class="list-decimal">
                    <li>"Implementierung eines generischen Komponenten zum interaktiven Rendern des Mandelbrot-Sets. Dies wurde mithilfe der Rust-Bibliothek \"wgpu\" umgesetzt. Diese bietet eine leichte Abstrahierung über mehrere Grafik-APIs (Vulkan, WebGL, OpenGL). So wurde ein Komponent geschrieben, welcher sowohl als native Anwendung, als auch als ein HTML-Canvas arbeiten kann."</li>
                    <li>"Erstellen der eigentlichen Fullstack-Anwendung in Rust mit dem Framework \"Leptos\", welches außerdem Server-Side-Rendering unterstützt."</li>
                </ol>

                <br/>
                "Eine Liste aller genutzten nennenswerten Technologien:"
                <ul class="list-disc">
                    <li>"Programmiersprache: Rust"</li>
                    <li>"Fullstack Framework: Leptos"</li>
                    <li>"Datenbank: PostgreSQL"</li>
                    <li>"ORM für erleichterten Datenbankzugriff: Diesel.rs"</li>
                    <li>"Frontend Styling: TailwindCSS"</li>
                    <li>"Containerisierung: Docker"</li>
                    <li>"Mandelbrot Rendering: WPGU für das platform-unabhängigen Rendering auf der Grafikkarte"</li>
                </ul>
            </p>


            <h3 class="font-bold underline">Fazit: Verifikation und Validierung</h3>
            <p>
                "Abschließend lässt sich zusammenfassen, dass das Projekt, wie es durch die Mockups und das Flowchart beschrieben wurde, umgesetzt werden konnte. "
                "Es wurde Wert darauf gelegt, keine groben Sicherheitslücken (mit Ausnahme der unsignierten JWT-Tokens) zu lassen. "
                "Die Sprache Rust hilft dahingehenend, wenige untentdeckte Fehler zu produzieren, da sie mit ihrem starken Typsystem dazu zwingt, alle Fälle zu behandeln. "

            </p>

            <h3 class="font-bold underline">Kommentare zur Sicherheit</h3>
            <ul class="list-disc">
                <li>"Es werden aktuell nur unsignierte \"JWT-Tokens\" verwendet, da die Implementierung dieser aufgrund meiner Technologiewahl einige Hürden mit sich bringt."</li>
                <li>"Beim Start eines Spiels werden dem Client die Start-Koordinaten in Klartext übertragen. Es hindert also niemand den Nutzer daran, sie in den Entwicklungstools auszulesen und somit immer ein richtiges Ergebnis erzieheln zu können. Dieses Problem lässt sich nur lösen, indem die zu suchenden Mandelbrot-Ausschnitte auf dem Server oder bereits im Voraus gerendert werden."</li>
                <li>"Speicherung von Passwörtern aktuell durch unsicheren md5 Hash. Sicherere Hashfunktion + Salting + Pepper möglich."</li>
            </ul>
        </div>
    }
}
