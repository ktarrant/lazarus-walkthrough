<!-- area-id: kaptara-island-west -->
### Kaptara Island (West)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=altaria">Altaria</a> | 5% | 5% | <input type="checkbox" class="caught-check" data-species="altaria" /> |
| <a href="../pokemon-lookup.html?q=duosion">Duosion</a> | 8% | — | <input type="checkbox" class="caught-check" data-species="duosion" /> |
| <a href="../pokemon-lookup.html?q=fomantis">Fomantis</a> | 20% | 20% | <input type="checkbox" class="caught-check" data-species="fomantis" /> |
| <a href="../pokemon-lookup.html?q=lurantis">Lurantis</a> | 5% | 5% | <input type="checkbox" class="caught-check" data-species="lurantis" /> |
| <a href="../pokemon-lookup.html?q=morgrem">Morgrem</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="morgrem" /> |
| <a href="../pokemon-lookup.html?q=oranguru">Oranguru</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="oranguru" /> |
| <a href="../pokemon-lookup.html?q=passimian">Passimian</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="passimian" /> |
| <a href="../pokemon-lookup.html?q=reuniclus">Reuniclus</a> | 2% | — | <input type="checkbox" class="caught-check" data-species="reuniclus" /> |
| <a href="../pokemon-lookup.html?q=trumbeak">Trumbeak</a> | 20% | 20% | <input type="checkbox" class="caught-check" data-species="trumbeak" /> |
| <a href="../pokemon-lookup.html?q=yanma">Yanma</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="yanma" /> |
| <a href="../pokemon-lookup.html?q=claydol">Claydol</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="claydol" /> |
| <a href="../pokemon-lookup.html?q=espathra">Espathra</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="espathra" /> |
| <a href="../pokemon-lookup.html?q=golduck">Golduck</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="golduck" /> |
| <a href="../pokemon-lookup.html?q=gothitelle">Gothitelle</a> | — | 2% | <input type="checkbox" class="caught-check" data-species="gothitelle" /> |
| <a href="../pokemon-lookup.html?q=gothorita">Gothorita</a> | — | 8% | <input type="checkbox" class="caught-check" data-species="gothorita" /> |

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

