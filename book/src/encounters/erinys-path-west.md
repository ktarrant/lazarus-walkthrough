<!-- area-id: erinys-path-west -->
### Erinys Path (West)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=baltoy">Baltoy</a> | 10% | 20% | <input type="checkbox" class="caught-check" data-species="baltoy" /> |
| <a href="../pokemon-lookup.html?q=komala">Komala</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="komala" /> |
| <a href="../pokemon-lookup.html?q=minior-core">Minior Blue Core</a> | 4% | — | <input type="checkbox" class="caught-check" data-species="minior-core" /> |
| <a href="../pokemon-lookup.html?q=minior-core">Minior Indigo Core</a> | 4% | — | <input type="checkbox" class="caught-check" data-species="minior-core" /> |
| <a href="../pokemon-lookup.html?q=omanyte">Omanyte</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="omanyte" /> |
| <a href="../pokemon-lookup.html?q=sandile">Sandile</a> | 20% | — | <input type="checkbox" class="caught-check" data-species="sandile" /> |
| <a href="../pokemon-lookup.html?q=solosis">Solosis</a> | 20% | — | <input type="checkbox" class="caught-check" data-species="solosis" /> |
| <a href="../pokemon-lookup.html?q=togepi">Togepi</a> | 2% | 2% | <input type="checkbox" class="caught-check" data-species="togepi" /> |
| <a href="../pokemon-lookup.html?q=zigzagoon">Zigzagoon</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="zigzagoon" /> |
| <a href="../pokemon-lookup.html?q=zorua">Zorua</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="zorua" /> |
| <a href="../pokemon-lookup.html?q=galarian-zigzagoon">Galarian Zigzagoon</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="galarian-zigzagoon" /> |
| <a href="../pokemon-lookup.html?q=gothita">Gothita</a> | — | 20% | <input type="checkbox" class="caught-check" data-species="gothita" /> |
| <a href="../pokemon-lookup.html?q=minior-core">Minior Orange Core</a> | — | 4% | <input type="checkbox" class="caught-check" data-species="minior-core" /> |
| <a href="../pokemon-lookup.html?q=minior-core">Minior Red Core</a> | — | 4% | <input type="checkbox" class="caught-check" data-species="minior-core" /> |
| <a href="../pokemon-lookup.html?q=stantler">Stantler</a> | — | 20% | <input type="checkbox" class="caught-check" data-species="stantler" /> |
| <a href="../pokemon-lookup.html?q=hisuian-zorua">Zorua Hisuian</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="hisuian-zorua" /> |

<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>

