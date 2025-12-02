<!-- area-id: acrisia-mountains -->
### Acrisia Mountains

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=aron">Aron</a> | 8% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="aron" /> |
| <a href="../pokemon-lookup.html?q=klawf">Klawf</a> | 2% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="klawf" /> |
| <a href="../pokemon-lookup.html?q=mankey">Mankey</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="mankey" /> |
| <a href="../pokemon-lookup.html?q=mawile">Mawile</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="mawile" /> |
| <a href="../pokemon-lookup.html?q=nosepass">Nosepass</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="nosepass" /> |
| <a href="../pokemon-lookup.html?q=rockruff">Rockruff</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="rockruff" /> |
| <a href="../pokemon-lookup.html?q=sandshrew">Sandshrew</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="sandshrew" /> |
| <a href="../pokemon-lookup.html?q=timburr">Timburr</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="timburr" /> |
| <a href="../pokemon-lookup.html?q=vullaby">Vullaby</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="vullaby" /> |
| <a href="../pokemon-lookup.html?q=woobat">Woobat</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="woobat" /> |
| <a href="../pokemon-lookup.html?q=alolan-sandshrew">Alolan Sandshrew</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="alolan-sandshrew" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | 10% | — | — | 40% | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | 30% | — | — | 40% | <input type="checkbox" class="caught-check" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=wimpod">Wimpod</a> | — | — | 60% | 30% | — | — | <input type="checkbox" class="caught-check" data-species="wimpod" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=poliwag">Poliwag</a> | — | — | — | — | 60% | — | <input type="checkbox" class="caught-check" data-species="poliwag" /> |
| <a href="../pokemon-lookup.html?q=wooper">Wooper</a> | — | — | — | — | 40% | — | <input type="checkbox" class="caught-check" data-species="wooper" /> |

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

