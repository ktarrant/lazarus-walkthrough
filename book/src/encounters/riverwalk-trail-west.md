<!-- area-id: riverwalk-trail-west -->
### Riverwalk Trail (West)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=aron">Aron</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="aron" /> |
| <a href="../pokemon-lookup.html?q=cutiefly">Cutiefly</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cutiefly" /> |
| <a href="../pokemon-lookup.html?q=gligar">Gligar</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="gligar" /> |
| <a href="../pokemon-lookup.html?q=jangmo-o">Jangmo-o</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="jangmo-o" /> |
| <a href="../pokemon-lookup.html?q=litleo">Litleo</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="litleo" /> |
| <a href="../pokemon-lookup.html?q=meditite">Meditite</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="meditite" /> |
| <a href="../pokemon-lookup.html?q=mudbray">Mudbray</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="mudbray" /> |
| <a href="../pokemon-lookup.html?q=ponyta">Ponyta</a> | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="ponyta" /> |
| <a href="../pokemon-lookup.html?q=sandile">Sandile</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="sandile" /> |
| <a href="../pokemon-lookup.html?q=yamper">Yamper</a> | 8% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="yamper" /> |
| <a href="../pokemon-lookup.html?q=cubchoo">Cubchoo</a> | — | 8% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cubchoo" /> |
| <a href="../pokemon-lookup.html?q=galarian-ponyta">Galarian Ponyta</a> | — | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="galarian-ponyta" /> |
| <a href="../pokemon-lookup.html?q=yanma">Yanma</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="yanma" /> |
| <a href="../pokemon-lookup.html?q=clauncher">Clauncher</a> | — | — | 60% | — | 60% | — | <input type="checkbox" class="caught-check" data-species="clauncher" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | 30% | — | — | 40% | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | 10% | — | — | 40% | <input type="checkbox" class="caught-check" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=poliwag">Poliwag</a> | — | — | — | 30% | — | — | <input type="checkbox" class="caught-check" data-species="poliwag" /> |
| <a href="../pokemon-lookup.html?q=buizel">Buizel</a> | — | — | — | — | 40% | — | <input type="checkbox" class="caught-check" data-species="buizel" /> |
| <a href="../pokemon-lookup.html?q=clawitzer">Clawitzer</a> | — | — | — | — | — | 20% | <input type="checkbox" class="caught-check" data-species="clawitzer" /> |

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

