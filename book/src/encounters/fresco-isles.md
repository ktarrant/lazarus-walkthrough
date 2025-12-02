<!-- area-id: fresco-isles -->
### Fresco Isles

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=cyclizar">Cyclizar</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cyclizar" /> |
| <a href="../pokemon-lookup.html?q=doduo">Doduo</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="doduo" /> |
| <a href="../pokemon-lookup.html?q=gligar">Gligar</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="gligar" /> |
| <a href="../pokemon-lookup.html?q=helioptile">Helioptile</a> | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="helioptile" /> |
| <a href="../pokemon-lookup.html?q=krokorok">Krokorok</a> | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="krokorok" /> |
| <a href="../pokemon-lookup.html?q=larvitar">Larvitar</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="larvitar" /> |
| <a href="../pokemon-lookup.html?q=numel">Numel</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="numel" /> |
| <a href="../pokemon-lookup.html?q=scraggy">Scraggy</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="scraggy" /> |
| <a href="../pokemon-lookup.html?q=skorupi">Skorupi</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="skorupi" /> |
| <a href="../pokemon-lookup.html?q=tinkatuff">Tinkatuff</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="tinkatuff" /> |
| <a href="../pokemon-lookup.html?q=trapinch">Trapinch</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="trapinch" /> |
| <a href="../pokemon-lookup.html?q=dreepy">Dreepy</a> | — | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="dreepy" /> |
| <a href="../pokemon-lookup.html?q=paldean-wooper">Paldean Wooper</a> | — | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="paldean-wooper" /> |
| <a href="../pokemon-lookup.html?q=salandit">Salandit</a> | — | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="salandit" /> |
| <a href="../pokemon-lookup.html?q=dratini">Dratini</a> | — | — | 5% | — | — | 5% | <input type="checkbox" class="caught-check" data-species="dratini" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | 5% | — | — | — | <input type="checkbox" class="caught-check" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | 60% | 30% | — | — | <input type="checkbox" class="caught-check" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | 30% | — | — | 40% | <input type="checkbox" class="caught-check" data-species="wailmer" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="caught-check" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 20% | — | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | <input type="checkbox" class="caught-check" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=chinchou">Chinchou</a> | — | — | — | — | — | 15% | <input type="checkbox" class="caught-check" data-species="chinchou" /> |

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

