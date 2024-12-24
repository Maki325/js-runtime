import './impl.js';

async function Coffee({ hotOrIced, children }) {
  // const url = `https://api.sampleapis.com/coffee/${hotOrIced}`;
  // const res = await fetch(url);
  // const coffees = await res.json();
  await new Promise((resolve) => setTimeout(() => resolve(), 3_000));
  return (() => {
    const _6DagR6BQRUlJATr7 = [];
    return [
      `<ul>

    ${globalThis.___FRAMEWORK_JS_STRINGIFY___(COFFEES.map((coffee) => (() => {
        const _6DagR6BQRUlJATr7 = [];
        return [
          `<li>${globalThis.___FRAMEWORK_JS_STRINGIFY___(coffee.title, _6DagR6BQRUlJATr7)}</li>`,
          (_NQP2HsEt3OzZ) => {
            const _UXjTmqUnApme = _6DagR6BQRUlJATr7.map((_KkgF6w7893EX) => _KkgF6w7893EX(_NQP2HsEt3OzZ));
            return Promise.allSettled(_UXjTmqUnApme);
          }
        ];
      })()), _6DagR6BQRUlJATr7)}

    ${globalThis.___FRAMEWORK_JS_STRINGIFY___(children, _6DagR6BQRUlJATr7)}

  </ul>`,
      (_Qs2ut0QJU2yb) => {
        const _SF2kAAVDTmZl = _6DagR6BQRUlJATr7.map((_rnucWoJgWrrt) => _rnucWoJgWrrt(_Qs2ut0QJU2yb));
        return Promise.allSettled(_SF2kAAVDTmZl);
      }
    ];
  })();
}
const common = {
  Hello: function Hello({ name }) {
    const fontSize = '2em';
    const bStyle = {
      color: 'var(--test)',
      '--aa': '"hello"'
    };
    return (() => {
      const _6DagR6BQRUlJATr7 = [];
      return [
        `<p style="font-size: ${globalThis.___FRAMEWORK_JS_STYLE_VALUE___(fontSize, "fontSize")};color: red;margin: 0;padding: 0;--test: #1234AA">

      Hello

      <b style="background-color: #fefefe;border-radius: 10px;padding: 0 5;${globalThis.___FRAMEWORK_JS_STYLE_OBJECT___(bStyle)}">${globalThis.___FRAMEWORK_JS_STRINGIFY___(name, _6DagR6BQRUlJATr7)}</b>!

    </p>`,
        (_I54ONVap5FHL) => {
          const _PidxVUe5x6DW = _6DagR6BQRUlJATr7.map((_BjJJnl9Rf4Hk) => _BjJJnl9Rf4Hk(_I54ONVap5FHL));
          return Promise.allSettled(_PidxVUe5x6DW);
        }
      ];
    })();
  }
};
function HTML({ title, children }) {
  return (() => {
    const _6DagR6BQRUlJATr7 = [];
    return [
      `<!DOCTYPE html>
    <html>

    <head>${globalThis.___FRAMEWORK_JS_STRINGIFY___(title ? (() => {
        const _6DagR6BQRUlJATr7 = [];
        return [
          `<title>${globalThis.___FRAMEWORK_JS_STRINGIFY___(title, _6DagR6BQRUlJATr7)}</title>`,
          (_t6aVdRxWU9Ef) => {
            const _53Um8TkqeJ7a = _6DagR6BQRUlJATr7.map((_6fbVbcrpAy1f) => _6fbVbcrpAy1f(_t6aVdRxWU9Ef));
            return Promise.allSettled(_53Um8TkqeJ7a);
          }
        ];
      })() : null, _6DagR6BQRUlJATr7)}</head>

    <body style="background-color: #121212;color: white">

      ${globalThis.___FRAMEWORK_JS_STRINGIFY___(children, _6DagR6BQRUlJATr7)}

    </body>

  </html>`,
      (_XqQFykHloCdw) => {
        const _WxE4e4up9tYz = _6DagR6BQRUlJATr7.map((_F0eSyHDCTSuZ) => _F0eSyHDCTSuZ(_XqQFykHloCdw));
        return Promise.allSettled(_WxE4e4up9tYz);
      }
    ];
  })();
}

export default async function Page() {
  return (() => {
    const _6DagR6BQRUlJATr7 = [];
    return [
      globalThis.___FRAMEWORK_JS_STRINGIFY___(HTML({
        children: '\n\n      <div id="_ewoxENzZd9FL"></div>\n\n      <div id="_nd6roKlax2Vk"></div>\n\n    '
      }), _6DagR6BQRUlJATr7),
      (_ohjFTjErI1B7) => {
        const _eiTqiRSxkrqI = _6DagR6BQRUlJATr7.map((_8bIf5UwuoFBX) => _8bIf5UwuoFBX(_ohjFTjErI1B7));
        _eiTqiRSxkrqI.push((async () => {
          const [_gDySm5DoOXMb, _vrMhr5RfqbvE] = await common.Hello({
            children: "",
            name: "Marko"
          });
          _ohjFTjErI1B7.enqueue(`<script id="_8GafSAoEKZYK">document.getElementById("_ewoxENzZd9FL").outerHTML = \`${_gDySm5DoOXMb.replace(/`/mg, "\\`")}\`;document.getElementById("_8GafSAoEKZYK"    ).remove();</script>`);
          _vrMhr5RfqbvE(_ohjFTjErI1B7);
        })());
        _eiTqiRSxkrqI.push((async () => {
          const [_gDySm5DoOXMb, _vrMhr5RfqbvE] = await Coffee({
            children: "\n\n        <h1>STUFF`</h1>\n\n      ",
            hotOrIced: "iced"
          });
          _ohjFTjErI1B7.enqueue(`<script id="_7GRL2b2pD5X1">document.getElementById("_nd6roKlax2Vk").outerHTML = \`${_gDySm5DoOXMb.replace(/`/mg, "\\`")}\`;document.getElementById("_7GRL2b2pD5X1"    ).remove();</script>`);
          _vrMhr5RfqbvE(_ohjFTjErI1B7);
        })());
        return Promise.allSettled(_eiTqiRSxkrqI);
      }
    ];
  })();
}

const COFFEES = [
  {
    "title": "Black Coffee",
    "description": "Svart kaffe är så enkelt som det kan bli med malda kaffebönor dränkta i hett vatten, serverat varmt. Och om du vill låta fancy kan du kalla svart kaffe med sitt rätta namn: café noir.",
    "ingredients": [
      "Coffee"
    ],
    "image": "https://images.unsplash.com/photo-1494314671902-399b18174975?auto=format&fit=crop&q=80&w=1887&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
    "id": 1
  },
  {
    "title": "Latte",
    "description": "Som den mest populära kaffedrycken där ute består latte av en skvätt espresso och ångad mjölk med bara en gnutta skum. Den kan beställas utan smak eller med smak av allt från vanilj till pumpa kryddor.",
    "ingredients": [
      "Espresso",
      "Ångad mjölk"
    ],
    "image": "https://images.unsplash.com/photo-1561882468-9110e03e0f78?auto=format&fit=crop&q=60&w=800&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTl8fGxhdHRlfGVufDB8fDB8fHww",
    "id": 2
  },
  {
    "title": "Caramel Latte",
    "description": "Om du gillar latte med en speciell smak kan karamell latte vara det bästa alternativet för att ge dig en upplevelse av den naturliga sötman och krämigheten hos ångad mjölk och karamell.",
    "ingredients": [
      "Espresso",
      "Ångad mjölk",
      "Karamellsirap"
    ],
    "image": "https://images.unsplash.com/photo-1599398054066-846f28917f38?auto=format&fit=crop&q=80&w=1887&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
    "id": 3
  },
  {
    "title": "Cappuccino",
    "description": "Cappuccino är en latte som är gjord med mer skum än ångad mjölk, ofta med ett strö av kakaopulver eller kanel på toppen. Ibland kan du hitta variationer som använder grädde istället för mjölk eller sådana som tillsätter smakämnen också.",
    "ingredients": [
      "Espresso",
      "Ångad mjölk",
      "Foam"
    ],
    "image": "https://images.unsplash.com/photo-1557006021-b85faa2bc5e2?auto=format&fit=crop&q=80&w=1887&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
    "id": 4
  },
  {
    "title": "Americano",
    "description": "Med en liknande smak som svart kaffe består americano av en espresso skott utspätt med hett vatten.",
    "ingredients": [
      "Espresso",
      "Hett vatten"
    ],
    "image": "https://images.unsplash.com/photo-1532004491497-ba35c367d634?auto=format&fit=crop&q=80&w=1887&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
    "id": 5
  },
  {
    "title": "Espresso",
    "description": "Ett espressoskott kan serveras ensamt eller användas som grund för de flesta kaffedrycker, som latte och macchiato.",
    "ingredients": [
      "Espresso"
    ],
    "image": "https://images.unsplash.com/photo-1579992357154-faf4bde95b3d?auto=format&fit=crop&q=80&w=1887&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
    "id": 6
  },
  {
    "title": "Macchiato",
    "description": "Macchiaton är en annan espresso-baserad dryck som har en liten mängd skum på toppen. Det är det glada mellanrummet mellan en cappuccino och en doppio.",
    "ingredients": [
      "Espresso",
      "Foam"
    ],
    "image": "https://images.unsplash.com/photo-1557772611-722dabe20327?auto=format&fit=crop&q=80&w=1887&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
    "id": 7
  },
  {
    "title": "Mocha",
    "description": "För alla chokladälskare där ute kommer ni att bli förälskade i en mocha. Mocha är en choklad-espressodryck med ångad mjölk och skum.",
    "ingredients": [
      "Espresso",
      "Ångad mjölk",
      "Choklad"
    ],
    "image": "https://images.unsplash.com/photo-1607260550778-aa9d29444ce1?auto=format&fit=crop&q=80&w=1887&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
    "id": 8
  },
  {
    "title": "Hot Chocolate",
    "description": "Under kalla vinterdagar får en kopp varm choklad dig att känna dig bekväm och lycklig. Den får dig också att må bra eftersom den innehåller energigivande koffein.",
    "ingredients": [
      "Choklad",
      "Mjölk"
    ],
    "image": "https://images.unsplash.com/photo-1542990253-0d0f5be5f0ed?auto=format&fit=crop&q=60&w=800&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NDh8fGhvdCUyMGNob2NvbGF0ZXxlbnwwfHwwfHx8MA%3D%3D",
    "id": 9
  },
  {
    "title": "Chai Latte",
    "description": "Om du letar efter en smakfull varm dryck mitt i vintern, välj chai latte. Kombinationen av kardemumma och kanel ger en underbar smak.",
    "ingredients": [
      "Te",
      "Mjölk",
      "Ingefära",
      "Kardemumma",
      "Kanel"
    ],
    "image": "https://images.unsplash.com/photo-1578899952107-9c390f1af1b7?w=900&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTJ8fGNoYWklMjBsYXR0ZXxlbnwwfHwwfHx8MA%3D%3D",
    "id": 10
  },
  {
    "title": "Matcha Latte",
    "description": "Matcha latte är en grön, hälsosam kaffedryck med finkrossad matcha-te och mjölk, erbjuder mild sötma, en unik smak och en mild koffeinkick.",
    "ingredients": [
      "Matcha-pulver",
      "Mjölk",
      "Socker*"
    ],
    "image": "https://images.unsplash.com/photo-1536256263959-770b48d82b0a?w=900&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8M3x8bWF0Y2hhJTIwbGF0dGV8ZW58MHx8MHx8fDA%3D",
    "id": 11
  },
  {
    "title": "Seasonal Brew",
    "description": "Säsongs kaffe med olika smaktoner som karamell, frukt och choklad",
    "ingredients": [
      "Kaffe"
    ],
    "image": "https://images.unsplash.com/photo-1611162458324-aae1eb4129a4?w=900&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTg1fHxibGFjayUyMGNvZmZlZXxlbnwwfHwwfHx8MA%3D%3D",
    "id": 12
  },
  {
    "title": "Svart Te",
    "description": "Svart te föddes i Kina. Det är tillverkat av blad från en växt som kallas Camellia och kan smaksättas olika med frukter till exempel. En trevlig, varm, smakfull och aromatisk dryck som passar till vardagen.",
    "ingredients": [
      "Te"
    ],
    "image": "https://images.unsplash.com/photo-1576092768241-dec231879fc3?auto=format&fit=crop&q=60&w=800&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MjB8fHRlYXxlbnwwfHwwfHx8MA%3D%3D",
    "id": 13
  },
  {
    "title": "Islatte",
    "description": "Iced latte är en kyld kaffedryck som görs genom att blanda espresso och kyld mjölk. Den serveras med isbitar och är även känd som cafè latte iced eller latte on the rocks.",
    "ingredients": [
      "Espresso",
      "Mjölk",
      "Is",
      "Sirap"
    ],
    "image": "https://images.unsplash.com/photo-1517701550927-30cf4ba1dba5?auto=format&fit=crop&q=60&w=800&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8aWNlZCUyMGxhdHRlfGVufDB8fDB8fHww",
    "id": 14
  },
  {
    "title": "Islatte Mocha",
    "description": "Iced latte Mocha är en kombination av latte och mocha, som i sig är en kombination av choklad och kaffe. Den ger kalla dryckälskare en läcker upplevelse av choklad och kaffe.",
    "ingredients": [
      "Espresso",
      "Is",
      "Mjölk",
      "Choklad "
    ],
    "image": "https://images.unsplash.com/photo-1642647391072-6a2416f048e5?auto=format&fit=crop&q=60&w=800&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Mzh8fGljZWQlMjBtb2NoYSUyMGxhdHRlfGVufDB8fDB8fHww",
    "id": 15
  },
  {
    "title": "Frapino Caramel",
    "description": "Det är en blandad eller bättre sagt skakad kaffe med vispad grädde på toppen. Ett måste för varma sommardagar.",
    "ingredients": [
      "coffee",
      "Is",
      "Mjölk",
      "Karamellsirap",
      "Vispgrädde*",
      "Karamellsås"
    ],
    "image": "https://images.unsplash.com/photo-1662047102608-a6f2e492411f?auto=format&fit=crop&q=60&w=800&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8ZnJhcGlubyUyMGNhcmFtZWx8ZW58MHx8MHx8fDA%3D",
    "id": 16
  },
  {
    "title": "Frapino Mocka",
    "description": "Ännu en berömd och utsökt kall dryck för dem som föredrar choklad. Tänk dig smaken av en shake med choklad och vispad grädde på toppen.",
    "ingredients": [
      "Coffee",
      "Is",
      "Mjölk",
      "Cocoa",
      "Vispgrädde*"
    ],
    "image": "https://images.unsplash.com/photo-1530373239216-42518e6b4063?auto=format&fit=crop&q=60&w=800&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8ZnJhcGlubyUyMG1vY2hhfGVufDB8fDB8fHww",
    "id": 17
  },
  {
    "title": "Apelsinjuice",
    "description": "Vi har inget att säga om vår nypressade apelsinjuice. Du måste prova den själv.",
    "ingredients": [
      "Färska Apelsiner",
      "Is"
    ],
    "image": "https://images.unsplash.com/photo-1600271886742-f049cd451bba?auto=format&fit=crop&q=60&w=800&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NzF8fG9yYW5nZSUyMGp1aWNlfGVufDB8fDB8fHww",
    "id": 18
  },
  {
    "title": "Frozen Lemonade",
    "description": "Frozen lemonade är en uppfriskande sommardryck som kombinerar färskpressad citronsaft, is och sötning till en svalkande, syrlig och sötsyrlig smaksensation.",
    "ingredients": [
      "Citronsaft",
      "Is",
      "Socker*"
    ],
    "image": "https://images.unsplash.com/photo-1523371054106-bbf80586c38c?w=900&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTZ8fGxlbW9uYWRlJTIwd2l0aCUyMGljZXxlbnwwfHwwfHx8MA%3D%3D",
    "id": 19
  },
  {
    "title": "Lemonad",
    "description": "Var känd i Paris först och blev sedan mycket populär i hela Europa. Denna söta, färglösa, kolsyrade dryck görs genom att blanda citronsaft och kolsyrat vatten.",
    "ingredients": [
      "Citronsaft",
      "Kolsyrat vatten",
      "Honung"
    ],
    "image": "https://images.unsplash.com/photo-1621263764928-df1444c5e859?auto=format&fit=crop&q=60&w=800&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Nnx8bGVtb25hZGV8ZW58MHx8MHx8fDA%3D",
    "id": 20
  }
];
