<!-- area-id: sea-of-asteri-west -->
### Sea of Asteri (West)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | Underwater | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=bellsprout">Bellsprout</a> | 10% | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="bellsprout" /> |
| <a href="../pokemon-lookup.html?q=chatot">Chatot</a> | 10% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="chatot" /> |
| <a href="../pokemon-lookup.html?q=crabrawler">Crabrawler</a> | 15% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="crabrawler" /> |
| <a href="../pokemon-lookup.html?q=dreepy">Dreepy</a> | 2% | 2% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="dreepy" /> |
| <a href="../pokemon-lookup.html?q=fomantis">Fomantis</a> | 20% | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="fomantis" /> |
| <a href="../pokemon-lookup.html?q=scyther">Scyther</a> | 8% | 4% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="scyther" /> |
| <a href="../pokemon-lookup.html?q=stufful">Stufful</a> | 20% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="stufful" /> |
| <a href="../pokemon-lookup.html?q=wattrel">Wattrel</a> | 15% | 15% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="wattrel" /> |
| <a href="../pokemon-lookup.html?q=cufant">Cufant</a> | — | 15% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cufant" /> |
| <a href="../pokemon-lookup.html?q=ekans">Ekans</a> | — | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="ekans" /> |
| <a href="../pokemon-lookup.html?q=galarian-linoone">Galarian Linoone</a> | — | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="galarian-linoone" /> |
| <a href="../pokemon-lookup.html?q=heracross">Heracross</a> | — | 4% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="heracross" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | 60% | 30% | — | — | 20% | <input type="checkbox" class="caught-check" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | 10% | — | — | 40% | 20% | <input type="checkbox" class="caught-check" data-species="wailmer" /> |
| <a href="../pokemon-lookup.html?q=wingull">Wingull</a> | — | — | 30% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="wingull" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | 10% | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | — | <input type="checkbox" class="caught-check" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 20% | — | 20% | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | — | <input type="checkbox" class="caught-check" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=chinchou">Chinchou</a> | — | — | — | — | — | 15% | — | <input type="checkbox" class="caught-check" data-species="chinchou" /> |
| <a href="../pokemon-lookup.html?q=lanturn">Lanturn</a> | — | — | — | — | — | 5% | — | <input type="checkbox" class="caught-check" data-species="lanturn" /> |

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

