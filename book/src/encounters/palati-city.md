<!-- area-id: palati-city -->
### Palati City

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=archen">Archen</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="archen" /> |
| <a href="../pokemon-lookup.html?q=baltoy">Baltoy</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="baltoy" /> |
| <a href="../pokemon-lookup.html?q=girafarig">Girafarig</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="girafarig" /> |
| <a href="../pokemon-lookup.html?q=gurdurr">Gurdurr</a> | 4% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="gurdurr" /> |
| <a href="../pokemon-lookup.html?q=klawf">Klawf</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="klawf" /> |
| <a href="../pokemon-lookup.html?q=lycanroc-day">Lycanroc Day</a> | 2% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="lycanroc-day" /> |
| <a href="../pokemon-lookup.html?q=pawmo">Pawmo</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="pawmo" /> |
| <a href="../pokemon-lookup.html?q=quagsire">Quagsire</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="quagsire" /> |
| <a href="../pokemon-lookup.html?q=timburr">Timburr</a> | 4% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="timburr" /> |
| <a href="../pokemon-lookup.html?q=toedscool">Toedscool</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="toedscool" /> |
| <a href="../pokemon-lookup.html?q=clodsire">Clodsire</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="clodsire" /> |
| <a href="../pokemon-lookup.html?q=grafaiai">Grafaiai</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="grafaiai" /> |
| <a href="../pokemon-lookup.html?q=lycanroc-night">Lycanroc Night</a> | — | 2% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="lycanroc-night" /> |
| <a href="../pokemon-lookup.html?q=munna">Munna</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="munna" /> |
| <a href="../pokemon-lookup.html?q=toedscruel">Toedscruel</a> | — | 8% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="toedscruel" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | 60% | 30% | — | — | <input type="checkbox" class="caught-check" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=tirtouga">Tirtouga</a> | — | — | 10% | — | 60% | 15% | <input type="checkbox" class="caught-check" data-species="tirtouga" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | 30% | — | — | 40% | <input type="checkbox" class="caught-check" data-species="wailmer" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="caught-check" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | — | — | 20% | — | <input type="checkbox" class="caught-check" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=dhelmise">Dhelmise</a> | — | — | — | — | — | 5% | <input type="checkbox" class="caught-check" data-species="dhelmise" /> |

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

